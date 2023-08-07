use handler::handle_gravatar;
use job_scheduler::{Job, JobScheduler};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tokio;
use warp::{
    http::{Response, StatusCode},
    Filter,
};
use crate::util::*;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let gravatar_query = warp::query::<GravatarQuery>()
        .map(Some)
        .or_else(|_| async { Ok::<(Option<GravatarQuery>,), std::convert::Infallible>((None,)) });
    let gravatar = warp::get()
        .and(warp::path!("v2" / "avatar" / String))
        .and(gravatar_query)
        .and(warp::header("user-agent"))
        .and_then(handle_gravatar);

    warp::serve(hello.or(gravatar))
        .run(([0, 0, 0, 0], 5000))
        .await;
}

mod util;
mod global_config;
mod handler;