//#![feature(libc)]
mod main_test;
mod telldus;
mod sender;
mod receiver;
mod config;
mod extmsg;

extern crate libc;
extern crate rustc_serialize;
extern crate time;

use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};

#[derive(RustcEncodable, RustcDecodable)]
pub struct Config {
	clients: Vec<String>
}

#[allow(dead_code)]
fn main() {
	//uncomment to write new config. useful when changing the config signature
	//config::write_config(); return;
	
	let config = config::read_config().unwrap();

	let (tx, rx): (Sender<String>, Receiver<String>) = channel();

	let recv_handle = receiver::start(config.receiver_port, config.password, tx);
	let send_handle = sender::start(config.clients, rx);
	recv_handle.join().unwrap();
	send_handle.join().unwrap();
}

#[test]
fn test_retvalue() {
	assert!(main::retvalue()==2);
}
