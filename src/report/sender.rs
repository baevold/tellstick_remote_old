use report::telldus;
use std::thread;
use std::sync::mpsc::Receiver;
use std::net::UdpSocket;
use rustc_serialize::json::{self};
use common::telldus_types;
use report::internaltypes::SenderAction;

pub fn start(clients: Vec<String>, channel_receiver: Receiver<SenderAction>) -> thread::JoinHandle<()> {
	return thread::spawn(move || { start_sender(clients, channel_receiver); });
}

fn start_sender(clients: Vec<String>, channel_receiver: Receiver<SenderAction>) {
	let mut clients = clients.clone();
	loop {
		let action = channel_receiver.recv().unwrap();
		match action {
			SenderAction::Update => {},
			SenderAction::Register(newclient) => {
				clients = update_clients(&mut clients, newclient);
			}
		}
		for client in clients.clone() {
			send_status(get_status(), client);
		}
	}
}

fn update_clients(clients: &mut Vec<String>, newclient: String) -> Vec<String> {
	if validate_message(&newclient) {
		clients.push(newclient.clone());
		return clients.clone();
	} else {
		return clients.clone();
	}
}

fn get_status() -> telldus_types::Status {
	telldus::init();
	let status = telldus::get_status();
	telldus::close();
	return status;
}

fn send_status(status: telldus_types::Status, client: String) {
	let vec = client.split(":").collect::<Vec<&str>>();
	let ip = vec[0];
	let port = String::from(vec[1]).parse::<u16>().unwrap();
	let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
	let data: String = json::encode(&status).unwrap();
	println!("Sending {}", data);
	let buf = data.into_bytes();
	socket.send_to(&buf, (ip, port)).unwrap();
	drop(socket);
}

fn validate_message(msg: &String) -> bool {
	let vec = msg.split(":");
	if vec.count() == 2 { return true; }
	return false;
}
