use serde::{Deserialize, Serialize};
use std::fs::File;

use std::{env, fs};

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalConfig {
    pub root_path: String,
    pub image_quality: f32,
    pub gravatar_cdn: String,
    pub master_domain: String,
    pub update_circle: u32,
    pub picture_convert_threads: u32,
}

lazy_static::lazy_static! {

    pub static ref CONFIG: GlobalConfig = load_config();
}
pub fn load_config() -> GlobalConfig {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 && args[1] == "-C" {
        let mut file = File::open(args[2].as_str()).unwrap();
        let contents: String = String::from_utf8_lossy(&fs::read(args[2].as_str()).unwrap())
            .parse()
            .unwrap();
        toml::from_str(contents.as_str()).unwrap()
    } else {
        panic!("Cannot load config");
    }
}
