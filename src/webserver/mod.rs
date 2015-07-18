use std::thread;
use websocket::header::WebSocketProtocol;
use websocket::{Server, Message, Sender, Receiver};

mod webtypes;
mod config;

pub fn main() {
	println!("Hello from webserver");
	let config = config::read_config().unwrap();
	//config::write_config();
	handle_connections();
}

fn handle_connections() {
	let server = Server::bind("127.0.0.1:8876").unwrap();
	for connection in server {
		thread::spawn(move || {
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
						handle_message(msg);
					}
					_ => {}
				};
			}

		});
	}
}

fn handle_message(msg: String) {
	println!("{}", msg);
}
