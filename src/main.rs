mod model;

use crate::model::EventModel;
use serde_derive::Deserialize;
use std::fs;
use std::path::Path;
use std::process::exit;
use toml;

const CONFIGFILE: &str = "config.toml";

#[derive(Debug, Deserialize)]
struct Config {
    paths: Vec<RootPath>,
}

#[derive(Debug, Deserialize)]
struct RootPath {
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

    for path in config.paths {
        println!("path {:?}", path.filepath);
        let root_path = Path::new(&path.filepath);
        let entry = fs::read_dir(root_path);
        let entry = entry.unwrap();
        for item in entry.into_iter() {
            let item = item.unwrap();
            println!(
                "file name: {:?}\nfile type: {:?}\nfile metadata: {:?}\n",
                item.file_name(),
                item.file_type().unwrap(),
                item.metadata().unwrap()
            )
        }
    }
}
