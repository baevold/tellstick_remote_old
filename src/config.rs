use std::fs::File;
use std::io::prelude::*;
use rustc_serialize::json::{self};

static CONFIGFILE: &'static str = "cnf/report.json";

#[derive(RustcEncodable, RustcDecodable)]
pub struct Config {
	pub receiver_port: u16,
	pub password: String,
        pub clients: Vec<String>
}

#[allow(dead_code)]
pub fn write_config() {
        let clients = [String::from("someserver:8888")];
        let config = Config{ receiver_port: 8877, password: String::from("passwordhash"), clients: clients.to_vec() };
        let data: String = json::encode(&config).unwrap();
        println!("use echo '[data]' > jq . to prettyfi. Remember to quites!");
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
