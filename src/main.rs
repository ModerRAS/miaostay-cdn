use handler::{handle_gravatar, handle_image};

use crate::util::*;
use std::{convert::Infallible, time::Duration};


use actix_web::{web, App, HttpResponse, HttpServer, get, Responder};

use reqwest::{Client, StatusCode};
use tokio_stream::StreamExt;
use tracing::Span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// #[tokio::main]
#[actix_rt::main]
async fn main()->std::io::Result<()>{
   HttpServer::new(||{
        App::new()
            .service(handle_gravatar)
            .service(handle_image)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}


// async fn proxy_via_reqwest(State(client): State<Client>) -> Response {
//     let reqwest_response = match client.get("http://127.0.0.1:3000/stream").send().await {
//         Ok(res) => res,
//         Err(err) => {
//             tracing::error!(%err, "request failed");
//             return StatusCode::BAD_GATEWAY.into_response();
//         }
//     };

//     let mut response_builder = Response::builder().status(reqwest_response.status());

//     // This unwrap is fine because we haven't insert any headers yet so there can't be any invalid
//     // headers
//     *response_builder.headers_mut().unwrap() = reqwest_response.headers().clone();

//     response_builder
//         .body(Body::from_stream(reqwest_response.bytes_stream()))
//         // Same goes for this unwrap
//         .unwrap()
// }

// async fn stream_some_data() -> Body {
//     let stream = tokio_stream::iter(0..5)
//         .throttle(Duration::from_secs(1))
//         .map(|n| n.to_string())
//         .map(Ok::<_, Infallible>);
//     Body::from_stream(stream)
// } 
mod global_config;
mod handler;
mod util;
