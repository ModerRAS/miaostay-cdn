// use handler::{handle_gravatar, handle_image};

use std::{fmt, path::PathBuf, fs::remove_file};

use crate::handler::{get_target_mime, get_image, convert_image};


use tiny_http::{Server, Request, Header, Response, StatusCode};
use util::{file_reader_from_disk, file_writer_to_disk};

pub fn get_user_agent(headers: &[Header]) -> String {
    for header in headers {
        match header.field.as_str().as_str() {
            "User-Agent" => {
                println!("{:?}", header.value.as_str());
                return header.value.as_str().to_string();
            },
            _ => continue
        }
    }
    return String::from("");
}
pub fn get_referer(headers: &[Header]) -> String {
    for header in headers {
        match header.field.as_str().as_str() {
            "Referer" => {
                println!("{:?}", header.value.as_str());
                return header.value.as_str().to_string();
            },
            _ => continue
        }
    }
    return String::from("");
}
pub fn match_referer(referer: String, pattern: &String) -> bool {
    if referer.starts_with(pattern) {
        return true;
    }
    return false;
}

pub fn get_path(digist: &str, mime: &str) -> Option<PathBuf> {
    let mut path = PathBuf::new();
    path.push(&global_config::CONFIG.temp_path);
    match mime {
        "image/webp" =>  {
            path.push(format!("{}.webp", digist));
        },
        "image/jpeg" => {
            path.push(format!("{}.jpeg", digist));
        },
        _ => return Option::None
    }
    return Some(path);
}
pub async fn get_cache(digist: &str, mime: &str) -> Option<Vec<u8>> {
    let Some(path) = get_path(digist, mime) else {
        return None;
    };
    if path.exists() {
        let Ok(file) = file_reader_from_disk(&path).await else {
            return Option::None;
        };
        return Option::Some(file);
    }
    return Option::None;
}

pub async fn set_cache(file: &Vec<u8>, digist: &str, mime: &str) -> bool {
    let Some(path) = get_path(digist, mime) else {
        return false;
    };
    if path.exists() {
        remove_file(&path);
    }
    return file_writer_to_disk(file, &path).await;
}

pub async fn handle_request(request: Request) {
    let url = (&request).url().to_owned();
    let headers = (&request).headers().to_owned();
    println!("{:?}", &url);
    println!("accept {:?}", &headers);
    let ua = get_user_agent(&headers);
    let referer = get_referer(&headers);
    if !match_referer(referer, &global_config::CONFIG.master_domain) {
        let _ = request.respond(Response::empty(StatusCode::from(403)));
        return;
    }
    let digist = format!("{:x}", md5::compute((&url).as_bytes()));
    println!("md5 is {}", &digist);

    let target_mime = get_target_mime(ua);
    println!("{:?}", &target_mime);
    match get_cache(&digist, &target_mime).await {
        Some(image) => {
            let _ = request.respond(Response::from_data(image));
            return;
        },
        None => {
            let Ok(image) = get_image(url.to_string()).await else {
                println!("Cannot get image: {}", url.to_string());
                let _ = request.respond(Response::empty(StatusCode::from(500)));
                return;
            };
            let Ok(image) = convert_image(image.data, target_mime.clone()).await else {
                println!("Cannot convert image");
                let _ = request.respond(Response::empty(StatusCode::from(500)));
                return;
            };
            set_cache(&image.data, &digist, target_mime.as_str()).await;
            let _ = request.respond(Response::from_data(image.data));
            return;
        }
    }

    return;

}

#[tokio::main]
async fn main() {
    let server: Server = Server::http("0.0.0.0:5000").unwrap();
    loop {
        // blocks until the next request is received
        let request: Request = match server.recv() {
            Ok(rq) => rq,
            Err(e) => { println!("error: {}", e); break }
        };
        tokio::spawn(async move {
            handle_request(request).await;
        });


        // do something with the request
        // ...
    }
}

mod global_config;
mod handler;
mod util;