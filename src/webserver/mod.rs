use std::thread;
use websocket::header::WebSocketProtocol;
use websocket::{Server, Message, Sender, Receiver};
use std::str;
use std::sync::Arc;

use rustc_serialize::json::{self};

mod webtypes;
mod config;

static LOCALADDR: &'static str = "127.0.0.1";

pub fn main() {
	println!("Hello from webserver");
	let a: webtypes::Action = webtypes::Action::Login;
	let b: webtypes::Message = webtypes::Message{ hash: "hei".to_string(), action: a };
	let jstring = json::encode(&b).unwrap();
	println!("{}", jstring);
	let config = config::read_config().unwrap();
	//config::write_config();
	handle_connections(config);
}

fn handle_connections(config: config::Config) {
	let localaddr = format!("{}:{}", LOCALADDR, config.port);
	let server = Server::bind(str::from_utf8(localaddr.as_bytes()).unwrap()).unwrap();
	let config_arc = Arc::new(config);
	for connection in server {
		let config_clone = config_arc.clone();
		thread::spawn(move || {
			let hash = &config_clone.hash;
			let request = connection.unwrap().read_request().unwrap();
			let headers = request.headers.clone();
			request.validate().unwrap();

			let mut response = request.accept();

			if let Some(&WebSocketProtocol(ref protocols)) = headers.get() {
				if protocols.contains(&("rust-websocket".to_string())) {
					//protocol is ok
					println!("protocol is ok");
					response.headers.set(WebSocketProtocol(vec!["rust-websocket".to_string()]));
				}
			}
			let mut client = response.send().unwrap();
			let ip = client.get_mut_sender().get_mut().peer_addr().unwrap();

			println!("Connection from {}", ip);

			let (mut sender, mut receiver) = client.split();

			for message in receiver.incoming_messages() {
				let message = message.unwrap();

				match message {
					Message::Close(_) => {
						println!("Client {} disconnected", ip);
						return;
					}
					Message::Text(msg) => {
						match handle_message(msg, hash) {
							Some(text) => { sender.send_message(Message::Text(text)).unwrap(); }
							None => {}
						}
					}
					_ => {}
				};
			}

		});
	}
}

fn handle_message(msg: String, hash: &String) -> Option<String> {
	let message = webtypes::Message::from_string(msg.clone()).unwrap();
	match message.action {
		webtypes::Action::Login => {
			if str::from_utf8(message.hash.as_bytes()).unwrap() == str::from_utf8(hash.as_bytes()).unwrap() {
				println!("hash is correct");
				return Some(msg);
			} else {
				println!("hash is wrong");
				return None;
			}
		}
		webtypes::Action::RequestStatus => { return None; }
	}
}
