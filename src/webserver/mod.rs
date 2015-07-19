use std::thread;
use std::sync::Arc;
use std::sync::mpsc;
use rustc_serialize::json::{self};

mod webtypes;
mod config;
mod wshandler;
mod internaltypes;
mod statusreceiver;

pub fn main() {
	println!("Hello from webserver");
	let a: webtypes::Action = webtypes::Action::Login;
	let b: webtypes::Message = webtypes::Message{ hash: "hei".to_string(), action: a };
	let jstring = json::encode(&b).unwrap();
	println!("{}", jstring);
	let config = config::read_config().unwrap();
	let config_arc = Arc::new(config);
	//config::write_config();

	//channel for reporting back to main thread
	let (tx, rx) = mpsc::channel();

	//client connection threads
	let config_clients = config_arc.clone();
	let tx_clients = tx.clone();
	//let rx_clients = rx.clone();
	thread::spawn(move || { wshandler::handle_client_connections(&config_clients, tx_clients); });

	//thread receiving status updates
	let config_report = config_arc.clone();
	let tx_report = tx.clone();
	thread::spawn(move || { statusreceiver::receive_status(&config_report, tx_report); });
}


