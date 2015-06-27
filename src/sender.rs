use telldus;
use std::thread;
use std::sync::mpsc::Receiver;
use std::net::UdpSocket;
use time;
use rustc_serialize::json::{self};

static INTERVAL: u32 = 5000;

pub fn start(clients: Vec<String>, channel_receiver: Receiver<String>) -> thread::JoinHandle<()> {
	return thread::spawn(move || { start_sender(clients, channel_receiver); });
}

fn start_sender(clients: Vec<String>, channel_receiver: Receiver<String>) {
	let mut clients = clients.clone();
	loop {
		let start = time::SteadyTime::now();
		let recv_result = channel_receiver.try_recv();
		clients = match recv_result {
			Ok(v) => { if validate_message(v.clone()) { let mut l = clients.clone(); l.push(v.clone()); l} else { clients.clone() } },
			Err(_) => { clients.clone() }
		};
		for client in clients.clone() {
			send_status(get_status(), client);
		}
		let diff = (time::SteadyTime::now() - start).num_milliseconds();
		if diff < INTERVAL as i64 {
			thread::sleep_ms(INTERVAL-diff as u32);
		}
	}
}

fn get_status() -> telldus::Status {
	telldus::init();
	let status = telldus::get_status();
	telldus::close();
	return status;
}

fn send_status(status: telldus::Status, client: String) {
	let vec = client.split(":").collect::<Vec<&str>>();
	let ip = vec[0];
	let port = String::from(vec[1]).parse::<u16>().unwrap();
	let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
	let data: String = json::encode(&status).unwrap();
	println!("Sending {}", data);
	let buf = data.into_bytes();
	socket.send_to(&buf, (ip, port)).unwrap();
}

fn validate_message(msg: String) -> bool {
	let vec = msg.split(":");
	if vec.count() == 2 { return true; }
	return false;
}
