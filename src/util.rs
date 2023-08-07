use image::io::Reader as ImageReader;
use image::DynamicImage;
use serde::{Deserialize, Serialize};

use std::io::Cursor;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use webp::*;

use crate::global_config;

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

pub fn image_reader_from_buffer(buffer: &[u8]) -> Result<DynamicImage, ()> {
    // let Ok(source_img) = load_from_memory(buffer) else {return Err(())};
    let Ok(source_img) = ImageReader::new(Cursor::new(buffer)).with_guessed_format() else {return Err(())};
    let Ok(source_img) = source_img.decode() else { return Err(())};
    // let source_img = ImageReader::new(Cursor::new(buffer)).with_guessed_format().unwrap().decode().unwrap();
    Ok(source_img)
}
pub fn image_reader_from_disk(path: PathBuf) -> Result<DynamicImage, ()> {
    let Ok(source_img) = ImageReader::open(path) else { return Err(()) };
    let Ok(source_img) = source_img.decode() else {return Err(())};
    Ok(source_img)
}
pub fn convert_to_webp(source_img: DynamicImage) -> Result<Vec<u8>, ()> {
    // let mut bytes: Vec<u8> = Vec::new();
    let Ok(encoder) = Encoder::from_image(&source_img) else {return Err(())};
    let webp: WebPMemory = encoder.encode(global_config::CONFIG.image_quality);
    // let result = source_img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::WebP(75));
    Ok(webp.to_vec())
}
pub fn convert_to_jpg(source_img: DynamicImage) -> Result<Vec<u8>, ()> {
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
