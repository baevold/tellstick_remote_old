use std::fs::File;
use std::io::prelude::*;
use rustc_serialize::json::{self};

static CONFIGFILE: &'static str = "cnf/webserver.json";

#[derive(RustcEncodable, RustcDecodable)]
pub struct Config {
	pub hash: String,
	pub port: i32
}

#[allow(dead_code)]
pub fn write_config() {
	//hash is for user=a and password=a
	let hash = "98398f51aa78aaf6309be3d93ad27fb1c1b21cb6".to_string();
	let port = 8876;
        let config = Config{ hash: hash, port: port };
        let data: String = json::encode(&config).unwrap();
        println!("use echo '[data]' | jq . to prettyfi. Remember the quotes!");
        println!("{}", data);
}

pub fn read_config() -> Option<Config> {
        let mut file = match File::open(CONFIGFILE) {
                Ok(file) => file,
                Err(_) => panic!("no such file")
        };
        let mut json = String::new();
        match file.read_to_string(&mut json) {
                Ok(file) => file,
                Err(_) => panic!("could not read file to string")
        };
        return Some(json::decode(&json).unwrap());
}
