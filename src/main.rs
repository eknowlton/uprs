#[macro_use]
extern crate serde;
extern crate config;

mod settings;

use settings::Settings;
use std::path::PathBuf;

fn main() {
    let mut config_dir = PathBuf::new();
    config_dir.push("config");

    let config = Settings::new(config_dir).unwrap();

    println!("{:?}", config);
}
