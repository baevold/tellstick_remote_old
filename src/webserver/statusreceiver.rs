use std::sync::mpsc;
use std::sync::Arc;
use webserver::config;
use webserver::internaltypes;
use std::net::UdpSocket;
use common::telldus_types;
use std::str;

const RECEIVEBUFFERSIZE: usize = 1024;

pub fn receive_status(config: &Arc<config::Config>, tx: mpsc::Sender<internaltypes::InternalAction>) {
	//TODO port from config
	let addr = format!("0.0.0.0:{}", config.status_port);
	let socket_str = str::from_utf8(addr.as_bytes()).unwrap();
	let result = UdpSocket::bind(socket_str);
	let socket = match result {
			Err(_) => panic!("Couldnt create socket. {}", socket_str),
			Ok(v) => v
	};
	loop {
		let mut buffer = [0; RECEIVEBUFFERSIZE];
		let result = socket.recv_from(&mut buffer);
		// prepending with _ negates to unused warning. ouch
		let (_no_of_bytes, _) = match result {
				Err(_) => continue,
				Ok((a,b)) => (a,b)
		};
		let received_str = match str::from_utf8(&buffer) {
				Err(_) => { println!("Received non-utf8 data. Dropping it."); continue }, 
				Ok(v) => v
		};
		println!("{}", received_str);
		let status = match telldus_types::Status::from_string(received_str.to_string()) {
			Ok(v) => v,
			Err(_) => continue
		};
		tx.send(internaltypes::InternalAction::TellstickStatus(status)).unwrap();
	}
}
