use image::ImageBuffer;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use serde::{Deserialize, Serialize};

use crate::global_config::{self, REGEXES};
use std::io::Cursor;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use user_agent_parser::UserAgentParser;
use webp::*;

#[derive(Deserialize, Serialize, Clone)]
pub enum Rating {
    G,
    PG,
    R,
    X,
}

impl std::fmt::Display for Rating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Rating::G => write!(f, "G"),
            Rating::PG => write!(f, "PG"),
            Rating::R => write!(f, "R"),
            Rating::X => write!(f, "X"),
        }
    }
}

impl FromStr for Rating {
    type Err = ();

    fn from_str(input: &str) -> Result<Rating, Self::Err> {
        match input {
            "G" => Ok(Rating::G),
            "PG" => Ok(Rating::PG),
            "R" => Ok(Rating::R),
            "X" => Ok(Rating::X),
            _ => Err(()),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GravatarQuery {
    pub s: Option<u32>,
    pub r: Option<Rating>,
    pub d: Option<u32>,
}
pub async fn get_gravatar_image(
    uri: String,
    gravatar_query: GravatarQuery,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let gravatar_query_s = if let Some(gravatar_query_s) = gravatar_query.s {
        format!("s={}&", gravatar_query_s)
    } else {
        String::from("")
    };
    let gravatar_query_r = if let Some(gravatar_query_r) = gravatar_query.r {
        format!("r={}&", gravatar_query_r)
    } else {
        String::from("")
    };
    let gravatar_query_d = if let Some(gravatar_query_d) = gravatar_query.d {
        format!("d={}", gravatar_query_d)
    } else {
        String::from("")
    };

    let resp = reqwest::get(
        format!(
            "{}/{}?{}{}{}",
            global_config::CONFIG.gravatar_cdn,
            uri,
            gravatar_query_s,
            gravatar_query_r,
            gravatar_query_d
        )
        .trim_end_matches("&"),
    )
    .await?
    .bytes()
    .await?;

    println!("{:#?}", resp);
    Ok(resp.to_vec())
}

pub async fn get_gravatar_image_with_raw_query(uri: String, query_string: String) ->  Result<Vec<u8>, Box<dyn std::error::Error>> {
    let resp = reqwest::get(
        format!(
            "{}/{}?{}",
            global_config::CONFIG.gravatar_cdn,
            uri,
            query_string
        )
        .trim_end_matches("&"),
    )
    .await?
    .bytes()
    .await?;

    println!("{:#?}", resp);
    Ok(resp.to_vec())
}

pub async fn get_gravatar_image_with_raw_url(uri: String) ->  Result<Vec<u8>, Box<dyn std::error::Error>> {
    let resp = reqwest::get(
        format!(
            "{}/{}",
            global_config::CONFIG.gravatar_cdn,
            uri
        )
        .trim_end_matches("&"),
    )
    .await?
    .bytes()
    .await?;

    println!("{:#?}", resp);
    Ok(resp.to_vec())
}

pub fn image_reader_from_buffer(buffer: Vec<u8>) -> Result<DynamicImage, ()> {
    // let Ok(source_img) = load_from_memory(buffer) else {return Err(())};
    let Ok(source_img) = ImageReader::new(Cursor::new(buffer)).with_guessed_format() else {return Err(())};
    let Ok(source_img) = source_img.decode() else { return Err(())};
    // let source_img = ImageReader::new(Cursor::new(buffer)).with_guessed_format().unwrap().decode().unwrap();
    Ok(source_img)
}
pub async fn image_reader_from_disk(path: &PathBuf) -> Result<DynamicImage, ()> {
    let Ok(mut source_file) = File::open(path).await else {
        return Err(());
    };
    let mut contents = vec![];
    source_file.read_to_end(&mut contents).await;
    let Ok(source_img) = image_reader_from_buffer(contents) else { return Err(()) };
    
    // let Ok(source_img) = image::load_from_memory(&contents) else { return Err(()) };
    // let Ok(source_img) = source_img.decode() else {return Err(())};
    Ok(source_img)
}
pub fn convert_to_webp(source_img: &DynamicImage) -> Result<Vec<u8>, ()> {
    // let mut bytes: Vec<u8> = Vec::new();
    let Ok(encoder) = Encoder::from_image(source_img) else {return Err(())};
    let webp: WebPMemory = encoder.encode(global_config::CONFIG.image_quality);
    // let result = source_img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::WebP(75));
    Ok(webp.to_vec())
}
pub fn convert_to_jpg(source_img: &DynamicImage) -> Result<Vec<u8>, ()> {
    let mut bytes: Vec<u8> = Vec::new();
    let _ = source_img.write_to(
        &mut Cursor::new(&mut bytes),
        image::ImageOutputFormat::Jpeg(global_config::CONFIG.image_quality as u8),
    );
    Ok(bytes)
}
pub async fn save_to_file(bytes: Vec<u8>, path: PathBuf) -> Result<usize, ()> {
    let Ok(mut file) = File::create(&path).await else { return Err(())};

    // Writes some prefix of the byte string, not necessarily all of it.
    let Ok(size) = file.write(&bytes).await else { return Err(())};
    Ok(size)
}

pub fn is_support_webp(user_agent: String) -> bool {
    let ua_parser = UserAgentParser::from_str(REGEXES.as_str()).unwrap();
    let product = ua_parser.parse_product(user_agent.as_str());
    let Some(product_name) = product.name else {return false};
    let product_name = product_name.as_ref().clone();
    let Some(product_major) = product.major else {return false};
    let Ok(product_major) = str::parse::<i32>(product_major.as_ref().clone()) else { return false };
    let Some(product_minor) = product.minor else {return false};
    let Ok(product_minor)= str::parse::<i32>(product_minor.as_ref().clone()) else { return false };
    match product_name {
        "Edge" => {
            if product_major >= 18 {
                return true;
            } else {
                return false;
            }
        }
        "Chrome" | "Chrome Mobile WebView" => {
            if product_major >= 32 {
                return true;
            } else {
                return false;
            }
        }
        "Mobile Safari" => {
            if product_major >= 14 {
                return true;
            } else {
                return false;
            }
        }
        "Android" => {
            if product_major > 4 || product_major == 4 && product_minor > 2 {
                return true;
            } else {
                return false;
            }
        }
        "QQ Browser Mobile" => {
            if product_major >= 13 {
                return true;
            } else {
                return false;
            }
        }
        &_ => return false,
    }
}

#[cfg(test)]
mod test {
    use crate::global_config::REGEXES;
    use user_agent_parser::UserAgentParser;
    #[test]
    fn test_user_agent() {
        let ua_parser = UserAgentParser::from_str(REGEXES.as_str()).unwrap();
        let ua = " Mozilla/5.0 (iPad; U; CPU OS 4_2_1 like Mac OS X; zh-cn) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8C148 Safari/6533.18.5";
        let ios_ua = "Mozilla/5.0 (iPhone; CPU iPhone OS 16_5_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.5.1 Mobile/15E148 Safari/604.1";
        let edge_ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.188";
        let android_ua = "Mozilla/5.0 (Linux; U; Android 9; zh-cn; HWI-AL00 Build/HUAWEIHWI-AL00) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/66.0.3359.126 MQQBrowser/10.1 Mobile Safari/537.36";
        let android_ua2 = "Mozilla/5.0 (Linux; Android 9; MI 6 Build/PKQ1.190118.001; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/76.0.3809.89 Mobile Safari/537.36 T7/11.20 SP-engine/2.16.0 baiduboxapp/11.20.2.3 (Baidu; P1 9)";
        let product = ua_parser.parse_product(android_ua2);
        println!("{:#?}", product);
        println!("{}", product.name.unwrap().as_ref());
    }
}
