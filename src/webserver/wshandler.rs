use websocket::header::WebSocketProtocol;
use websocket::{Server, Message, Sender, Receiver};
use std::thread;
use std::sync::Arc;
use std::sync::mpsc;
use rustc_serialize::json::{self};
use std::str;
use webserver::webtypes;
use webserver::config;
use webserver::internaltypes;

//static LOCALADDR: &'static str = "127.0.0.1";
static LOCALADDR: &'static str = "0.0.0.0";

pub fn handle_client_connections(config: &Arc<config::Config>, tx: mpsc::Sender<internaltypes::InternalAction>) {
	let zt = webtypes::ZoneTemp { name: "hei".to_string(), temp: 1.0 };
	let st = webtypes::Action::SetTemp(zt);
	let sa = webtypes::Message{hash: "hash".to_string(), action: st };
	let stjson = json::encode(&sa).unwrap();
	println!("{}", stjson);
	let localaddr = format!("{}:{}", LOCALADDR, config.websocket_port);
	let server = Server::bind(str::from_utf8(localaddr.as_bytes()).unwrap()).unwrap();
	for connection in server {
		let config_clone = config.clone();
		let tx = tx.clone();
		thread::spawn(move || {
			let hash = &config_clone.hash;
			let request = connection.unwrap().read_request().unwrap();
			let headers = request.headers.clone();
			request.validate().unwrap();

			let mut response = request.accept();

			let mut validclient = false;

			if let Some(&WebSocketProtocol(ref protocols)) = headers.get() {
				if protocols.contains(&("rust-websocket".to_string())) {
					//protocol is ok
					response.headers.set(WebSocketProtocol(vec!["rust-websocket".to_string()]));
					validclient = true;
				}
			}
			if !validclient {
				return;
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
						match handle_message(msg, hash, &tx) {
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

fn handle_message(msg: String, hash: &String, tx: &mpsc::Sender<internaltypes::InternalAction>) -> Option<String> {
	let message = webtypes::Message::from_string(msg.clone()).unwrap();
	if str::from_utf8(message.hash.as_bytes()).unwrap() != str::from_utf8(hash.as_bytes()).unwrap() {
		println!("Got message with wrong hash. Silently ignoring");
		println!("{}", hash);
		return None;
	}
	match message.action {
		webtypes::Action::Login => {
			println!("Login successfull");
			return Some(msg);
		}
		webtypes::Action::RequestStatus => {
			println!("Got requeststatus {}", message.hash);
			let (mytx, myrx) = mpsc::channel();
			tx.send(internaltypes::InternalAction::RequestStatus(mytx)).unwrap();
			let status = myrx.recv().unwrap();
			let status_json = json::encode(&status).unwrap();
			println!("Sending {}", status_json);
			return Some(status_json);
		}
		webtypes::Action::Status(_) => {
			//shouldnt happen.
			println!("Got status! Something wrong...");
			return None;
		}
		webtypes::Action::SetTemp(zonetemp) => {
			println!("Setting new temp for {} to {}", zonetemp.name, zonetemp.temp);
			tx.send(internaltypes::InternalAction::SetTemp(zonetemp)).unwrap();
			return None;
		}
	}
}
