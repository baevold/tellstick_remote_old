use std::fs::File;
use std::io::prelude::*;
use rustc_serialize::json::{self};

static CONFIGFILE: &'static str = "cnf/webserver.json";
static DEFAULTMAPPINGFILE: &'static str = "cnf/mapping.json";
static LASTMAPPINGFILE: &'static str = "cnf/lastMapping.json";

#[derive(RustcEncodable, RustcDecodable)]
pub struct Config {
	pub hash: String,
	pub websocket_port: i32,
	pub status_port: i32,
	pub telldus_client: String,
	pub telldus_password: String
}

#[allow(dead_code)]
pub fn write_config() {
	//hash is for user=a and password=a
	let hash = "98398f51aa78aaf6309be3d93ad27fb1c1b21cb6".to_string();
	let port = 8876;
        let config = Config{ hash: hash, websocket_port: port, status_port: port-1, telldus_client: "localhost:8890".to_string(), telldus_password: "passord".to_string() };
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

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct Mapping {
	pub zones: Vec<Zone>
}

pub fn read_mapping() -> Option<Mapping> {
	let mut file = match File::open(LASTMAPPINGFILE) {
		Ok(file) => file,
		Err(_) => match File::open(DEFAULTMAPPINGFILE) {
                	Ok(file) => file,
                	Err(_) => panic!("no such file")
		}
        };
        let mut json = String::new();
        match file.read_to_string(&mut json) {
                Ok(file) => file,
                Err(_) => panic!("could not read file to string")
        };
        return Some(json::decode(&json).unwrap());
}

pub fn write_mapping(mapping: &Mapping, filename: &String) {
        let data: String = json::encode(&mapping).unwrap();
	let mut file = match File::create(filename) {
		Ok(file) => file,
		Err(_) => panic!("Could not create file {}",filename)
	};
	match file.write_all(data.as_bytes()) {
		Ok(_) => println!("Wrote to file!"),
		Err(_) => panic!("Could not write to file {}", filename)
	};
}

#[allow(dead_code)]
pub fn write_mapping_test() {
	let sw1 = Switch{ id: 1, name: "sw1".to_string() };
	let sw2 = Switch{ id: 2, name: "sw2".to_string() };
	let mut switches = Vec::new();
	switches.push(sw1);
	switches.push(sw2);
	let sw3 = Switch{ id: 3, name: "sw3".to_string() };
	let sw4 = Switch{ id: 4, name: "sw4".to_string() };
	let mut switches2 = Vec::new();
	switches2.push(sw3);
	switches2.push(sw4);
	let zone1 = Zone{ id: 1, name: "zone1".to_string(), target: 1.0, switches: switches };
	//let zone2 = Zone{ id: 2, name: "zone2".to_string(), target: 3.0, switches: switches2 };
	let mut zones = Vec::new();
	zones.push(zone1);
	//zones.push(zone2);
	let mapping = Mapping {zones: zones};
        let data: String = json::encode(&mapping).unwrap();
        println!("use echo '[data]' | jq . to prettyfi. Remember the quotes!");
        println!("{}", data);
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct Zone {
	pub id: i32,
	pub name: String,
	pub target: f32,
	pub switches: Vec<Switch>
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct Switch {
	pub id: i32,
	pub name: String
}
