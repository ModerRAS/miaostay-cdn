use warp::Rejection;

use crate::{util::{GravatarQuery, get_gravatar_image, is_support_webp, convert_to_webp, image_reader_from_buffer}, global_config::REGEXES};





pub async fn handle_gravatar(uri: String, p: Option<GravatarQuery>, user_agent: String) -> Result<Vec<u8>, Rejection> {
    match p {
        Some(obj) => {
            
            let Ok(image) = get_gravatar_image(uri.clone(), obj.clone()).await else { return Err(warp::reject())};
            if is_support_webp(user_agent) {
                let Ok(source_img) = image_reader_from_buffer(image) else {return Err(warp::reject())};
                let Ok(out_img) = convert_to_webp(source_img) else { return Err(warp::reject())};
                return Ok(out_img);
            }
            return Ok(image);
            // Response::builder().body(format!(
            //     "key1 = {}, key2 = {}",
            //     obj.s.unwrap(),
            //     obj.d.unwrap()
            // ))
        }
        None => {
            Err(warp::reject())
        }
    }
}