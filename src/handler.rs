use std::path::PathBuf;

use crate::{util::{is_support_webp, convert_to_webp, image_reader_from_buffer, get_gravatar_image_with_raw_query, image_reader_from_disk, convert_to_jpg, get_gravatar_image_with_raw_url}, global_config};

use tokio::{fs::File, io::AsyncReadExt};

pub struct ImageWithMime {
    pub Mime: String,
    pub Data: Vec<u8>
}

pub async fn get_image(url: String) -> Result<ImageWithMime, ()> {
    let url = urlencoding::decode(&url).expect("UTF-8").into_owned();
    let is_gravatar = url.starts_with("/v1/avatar");
    let is_image = url.starts_with("/v1/image");
    println!("is_gravatar: {:?}, is_image: {:?}", &is_gravatar, &is_image);
    if is_gravatar {
        let uri = url.replace("/v1/avatar/", "");
        let Ok(image) = get_gravatar_image_with_raw_url(uri).await else {
            return Err(())
        };
        return Ok(ImageWithMime{ Mime: "image/jpeg".to_string(), Data: image});
    } else if is_image {
        let uri = url.replace("/v1/image/", "");
        let mut file_path = PathBuf::new();
        file_path.push(&global_config::CONFIG.root_path);
        for per_path in uri.split("/") {
            let per_path = urlencoding::decode(per_path).expect("UTF-8").into_owned();
            file_path.push(per_path);
        }
        // let file_path_str = file_path.to_string();
        let Ok(mut source_file) = File::open(file_path).await else {
            return Err(());
        };
        let mut contents = vec![];
        source_file.read_to_end(&mut contents).await;
        let mime = mime_guess::from_path(&url).first().expect("image/jpeg");
        return Ok(ImageWithMime { Mime: mime.essence_str().to_string(), Data: contents });
    } else {
        return Err(());
    }
}

pub async fn convert_image(img: Vec<u8>, target_mime: String) -> Result<ImageWithMime, ()> {
    let Ok(img) = image_reader_from_buffer(img) else {
        return Err(());
    };
    match target_mime.as_str() {
        "image/webp" => {
            let Ok(img) = convert_to_webp(&img) else {
                return Err(());
            };
            Ok(ImageWithMime { Mime: "image/webp".to_string(), Data: img})
        }
        _ => {
            let Ok(img) = convert_to_jpg(&img) else {
                return Err(());
            };
            Ok(ImageWithMime { Mime: "image/jpeg".to_string(), Data: img})
        }
    }
}

pub fn get_target_mime(ua: String) -> String {
    if is_support_webp(ua) {
        return "image/webp".to_string();
    }
    return "image/jpeg".to_string();
}

// pub fn get_user_agent(request: &HttpRequest) -> Option<String> {
//     let Some(user_agent) = request.headers().get("User-Agent") else {
//         return Option::None;
//     };
//     let Ok(user_agent) = user_agent.to_str() else {
//         return Option::None;
//     };
//     let user_agent = user_agent.to_string();
//     return Some(user_agent);
// }

// #[get("/v1/avatar/{uri}")]
// pub async fn handle_gravatar(request: HttpRequest) -> HttpResponse {
//     let uri = request.uri().path().clone().replace("/v1/avatar/", "");
//     let query_string = request.query_string().to_string();
//     let Some(user_agent) = get_user_agent(&request) else {
//         return HttpResponse::BadRequest().body("cannot get user agent");
//     };
//     // uri: web::Path<String>, p: web::Query<GravatarQuery>, user_agent: web::Header<http::header::USER_AGENT>
//     let Ok(image) = get_gravatar_image_with_raw_query(uri.clone(), query_string.clone()).await else {
//         return HttpResponse::BadRequest().body("No Such Image");
//     };
//     println!("{}, {}, {}", uri, query_string, user_agent);
//     if is_support_webp(user_agent) {
//         let Ok(source_img) = image_reader_from_buffer(image) else {
//             return HttpResponse::BadRequest().body("cannot read image");
//         };
//         let Ok(out_img) = convert_to_webp(&source_img) else {
//             return HttpResponse::BadRequest().body("cannot convert to webp");
//         };
//         return HttpResponse::Ok().content_type("image/webp").body(out_img);
//     }
//     return HttpResponse::Ok().content_type("image/jpeg").body(image);
// }

// #[get("/v1/image/**")]
// pub async fn handle_image(request: HttpRequest) -> HttpResponse {
//     println!("accept");
//     let uri = request.uri().path().clone().replace("/v1/image/", "");
//     // let query_string = request.query_string().to_string();
//     let Some(user_agent) = get_user_agent(&request) else {
//         return HttpResponse::BadRequest().body("cannot get user agent");
//     };
//     // uri: web::Path<String>, p: web::Query<GravatarQuery>, user_agent: web::Header<http::header::USER_AGENT>
//     let mut file_path = PathBuf::new();
//     file_path.push(&global_config::CONFIG.root_path);
//     for per_path in uri.split("/") {
//         let per_path = urlencoding::decode(per_path).expect("UTF-8").into_owned();
//         file_path.push(per_path);
//     }
//     // let file_path_str = file_path.to_string();
//     let Ok(image) = image_reader_from_disk(&file_path).await else {
//         println!("{:?}", &file_path);
//         return HttpResponse::BadRequest().body("No Such Image");
//     };
//     println!("{}, {}", uri, user_agent);
//     if is_support_webp(user_agent) {
//         let Ok(out_img) = convert_to_webp(&image) else { 
//             return HttpResponse::BadRequest().body("cannot convert to webp");
//         };
//         return HttpResponse::Ok().content_type("image/webp").body(out_img);
//     } else {
//         let Ok(out_img) = convert_to_jpg(&image) else {
//             return HttpResponse::BadRequest().body("cannot convert to jpeg");
//         };
//         return HttpResponse::Ok().content_type("image/jpeg").body(out_img);
//     }
// }