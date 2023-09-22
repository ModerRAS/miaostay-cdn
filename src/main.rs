// use handler::{handle_gravatar, handle_image};

use crate::handler::{get_target_mime, get_image, convert_image};


use tiny_http::{Server, Request, Header, Response, StatusCode};

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
    let target_mime = get_target_mime(ua);
    println!("{:?}", &target_mime);
    let Ok(image) = get_image(url.to_string()).await else {
        println!("Cannot get image: {}", url.to_string());
        let _ = request.respond(Response::empty(StatusCode::from(500)));
        return;
    };
    let Ok(image) = convert_image(image.data, target_mime).await else {
        println!("Cannot convert image");
        let _ = request.respond(Response::empty(StatusCode::from(500)));
        return;
    };
    let _ = request.respond(Response::from_data(image.data));
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