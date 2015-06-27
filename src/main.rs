//#![feature(libc)]

mod main_test;
mod telldus;
extern crate libc;

use std::fs::File;
use std::io::prelude::*;

static CONFIGFILE: &'static str = "cnf/report.json";

pub mod main {
	pub fn retvalue() -> i32 {
		return 2;
	}
}

#[allow(dead_code)]
fn main() {
	telldus::init();

	let mut file = match File::open(CONFIGFILE) {
		Ok(file) => file,
		Err(_) => panic!("no such file")
	};
	let mut json = String::new();
	match file.read_to_string(&mut json) {
		Ok(file) => file,
		Err(_) => panic!("could not read file to string")
	};
	println!("{}", json);

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


#[test]
fn test_retvalue() {
	assert!(main::retvalue()==2);
}
