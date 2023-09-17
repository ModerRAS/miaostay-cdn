use std::path::PathBuf;

use actix_web::{get, HttpResponse, HttpRequest};

use crate::{util::{is_support_webp, convert_to_webp, image_reader_from_buffer, get_gravatar_image_with_raw_query, image_reader_from_disk, convert_to_jpg}, global_config};

pub fn get_user_agent(request: &HttpRequest) -> Option<String> {
    let Some(user_agent) = request.headers().get("User-Agent") else {
        return Option::None;
    };
    let Ok(user_agent) = user_agent.to_str() else {
        return Option::None;
    };
    let user_agent = user_agent.to_string();
    return Some(user_agent);
}

#[get("/v1/avatar/{uri}")]
pub async fn handle_gravatar(request: HttpRequest) -> HttpResponse {
    let uri = request.uri().path().clone().replace("/v1/avatar/", "");
    let query_string = request.query_string().to_string();
    let Some(user_agent) = get_user_agent(&request) else {
        return HttpResponse::BadRequest().body("cannot get user agent");
    };
    // uri: web::Path<String>, p: web::Query<GravatarQuery>, user_agent: web::Header<http::header::USER_AGENT>
    let Ok(image) = get_gravatar_image_with_raw_query(uri.clone(), query_string.clone()).await else { 
        return HttpResponse::BadRequest().body("No Such Image");
    };
    println!("{}, {}, {}", uri, query_string, user_agent);
    if is_support_webp(user_agent) {
        let Ok(source_img) = image_reader_from_buffer(image) else {
            return HttpResponse::BadRequest().body("cannot read image");
        };
        let Ok(out_img) = convert_to_webp(&source_img) else { 
            return HttpResponse::BadRequest().body("cannot convert to webp");
        };
        return HttpResponse::Ok().content_type("image/webp").body(out_img);
    }
    return HttpResponse::Ok().content_type("image/jpeg").body(image);
}

#[get("/v1/image/{uri}")]
pub async fn handle_image(request: HttpRequest) -> HttpResponse {
    let uri = request.uri().path().clone().replace("/v1/image/", "");
    // let query_string = request.query_string().to_string();
    let Some(user_agent) = get_user_agent(&request) else {
        return HttpResponse::BadRequest().body("cannot get user agent");
    };
    // uri: web::Path<String>, p: web::Query<GravatarQuery>, user_agent: web::Header<http::header::USER_AGENT>
    let mut file_path = PathBuf::new();
    file_path.push(&global_config::CONFIG.root_path);
    for per_path in uri.split("/") {
        file_path.push(per_path)
    }
    let Ok(image) = image_reader_from_disk(file_path).await else { 
        return HttpResponse::BadRequest().body("No Such Image");
    };
    println!("{}, {}", uri, user_agent);
    if is_support_webp(user_agent) {
        let Ok(out_img) = convert_to_webp(&image) else { 
            return HttpResponse::BadRequest().body("cannot convert to webp");
        };
        return HttpResponse::Ok().content_type("image/webp").body(out_img);
    } else {
        let Ok(out_img) = convert_to_jpg(&image) else {
            return HttpResponse::BadRequest().body("cannot convert to jpeg");
        };
        return HttpResponse::Ok().content_type("image/jpeg").body(out_img);
    }
}