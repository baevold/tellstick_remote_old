//#![feature(libc)]
mod main_test;
mod telldus;
mod sender;
mod receiver;
mod config;

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
	//config::write_config();
	let config = config::read_config().unwrap();

	let (tx, rx): (Sender<String>, Receiver<String>) = channel();

	let recv_handle = receiver::start(tx);
	let send_handle = sender::start(config.clients, rx);
	recv_handle.join().unwrap();
	send_handle.join().unwrap();
}

#[test]
fn test_retvalue() {
	assert!(main::retvalue()==2);
}
