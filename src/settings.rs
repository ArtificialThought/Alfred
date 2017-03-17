use std::fs::File;
use std::io::prelude::*;
use toml::{Parser, Value, Decoder};
use toml;

pub fn load(files: String) {
    for file in files {
        println!("> {}", i);
    }
}

struct Settings
{
    x:f64,
    y:f64
}


fn parse(path: String) -> Settings {
    let mut config_toml = String::new();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_)  => {
            error!("Could not find {} !", path);
            return Settings::new();
        }
    };

    file.read_to_string(&mut config_toml)
            .unwrap_or_else(|err| panic!("Error while reading settings: [{}]", err));

    let mut parser = Parser::new(&config_toml);
    let toml = parser.parse();

    if toml.is_none() {
        for err in &parser.errors {
            let (loline, locol) = parser.to_linecol(err.lo);
            let (hiline, hicol) = parser.to_linecol(err.hi);
            println!("{}:{}:{}-{}:{} error: {}",
                     path, loline, locol, hiline, hicol, err.desc);
        }
        panic!("Exiting server");
}


fn get() {

}


fn set() {

}
