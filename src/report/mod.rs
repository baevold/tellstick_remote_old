//#![feature(libc)]
mod report_test;
mod telldus;
mod sender;
mod receiver;
mod config;

use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};

#[allow(dead_code)]
pub fn main() {
	//uncomment to write new config. useful when changing the config signature
	//config::write_config(); return;
	//uncomment to write an extmsg::Message to stdout
	//extmsg::write_message(); return;
	
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
