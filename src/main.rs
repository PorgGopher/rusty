mod model;

use crate::model::EventModel;
use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;

const CONFIGFILE: &str = "config.toml";

#[derive(Debug, Deserialize)]
struct Config {
    path: Vec<Path>,
}

#[derive(Debug, Deserialize)]
struct Path {
    filepath: String,
}

fn main() {
    println!("Hello, world!!");

    let m = EventModel {
        filepath: "file.exe".to_string(),
        time: 1234,
    };

    println!("check out my model {:?}", m);

    let cfg = match fs::read_to_string(CONFIGFILE) {
        Ok(c) => c,
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file `{}`", CONFIGFILE);
            exit(1);
        }
    };

    let config: Config = match toml::from_str(&cfg) {
        Ok(d) => d,
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Unable to load data from `{}`", CONFIGFILE);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    for path in config.path {
        println!("path {:?}", path.filepath)
    }
}
