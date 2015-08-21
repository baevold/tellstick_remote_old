//#![feature(libc)]
mod report_test;
mod telldus;
mod sender;
mod receiver;
mod config;
mod internaltypes;

use std::sync::mpsc::channel;
use std::thread;

#[allow(dead_code)]
pub fn main() {
	//uncomment to write new config. useful when changing the config signature
	//config::write_config(); return;
	//uncomment to write an extmsg::Message to stdout
	//extmsg::write_message(); return;
	
	let config = config::read_config().unwrap();
	telldus::init();

	let (tx, rx) = channel();
	let timertx = tx.clone();

	let recv_handle = receiver::start(config.receiver_port, config.password, tx);
	let send_handle = sender::start(config.clients, rx);
	// regular updates are initiated here. a simple timer which notifies the sender
	let update_thread = thread::spawn(move || {
		loop {
			timertx.send(internaltypes::SenderAction::Update).unwrap();
			thread::sleep_ms(5000 as u32);
		}
	});
	recv_handle.join().unwrap();
	send_handle.join().unwrap();
	update_thread.join().unwrap();
	telldus::close();
}

#[test]
fn test_retvalue() {
	assert!(main::retvalue()==2);
}
