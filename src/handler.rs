use warp::Rejection;

use crate::util::{GravatarQuery, get_gravatar_image};



pub async fn handle_gravatar(uri: String, p: Option<GravatarQuery>, agent: String) -> Result<String, Rejection> {
    match p {
        Some(obj) => {
            let Ok(image) = get_gravatar_image(uri.clone(), obj.clone()).await else { return Err(warp::reject())};
            Ok(String::from(""))
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