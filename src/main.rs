//#![feature(libc)]

mod main_test;
mod telldus;
extern crate libc;
extern crate rustc_serialize;

use std::fs::File;
use std::io::prelude::*;
use rustc_serialize::json::{self};

static CONFIGFILE: &'static str = "cnf/report.json";

pub mod main {
	pub fn retvalue() -> i32 {
		return 2;
	}
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Config {
	clients: Vec<String>
}

#[allow(dead_code)]
fn main() {
	telldus::init();

	let config = read_config();
	//write_config();

	let sensors = telldus::get_sensors();
	for sensor in sensors {
		println!("{}", sensor.to_string());
	}
	let devices = telldus::get_devices();
	for device in devices {
		println!("{}", device.to_string());
	}
	telldus::close();
}

#[allow(dead_code)]
fn write_config() {
	let clients = [String::from("baekkevold.net:8876")];
	let config = Config{ clients: clients.to_vec() };
	let data: String = json::encode(&config).unwrap();
	println!("use echo '[data]' > jq . to prettyfi");
	println!("{}", data);
}

fn read_config() -> Option<Config> {
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

#[test]
fn test_retvalue() {
	assert!(main::retvalue()==2);
}
