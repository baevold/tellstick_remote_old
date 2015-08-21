use std::thread;
use std::sync::mpsc::Sender;
use std::net::UdpSocket;
use std::str;
use common::extmsg;
use report::telldus;
use report::internaltypes::SenderAction;

const INTERVAL: u32 = 3000;
const RECEIVEBUFFERSIZE: usize = 4096;

pub fn start(port: u16, password: String, channel_sender: Sender<SenderAction>) -> thread::JoinHandle<()> {
	return thread::spawn(move || { start_receiver(port, password, channel_sender); });
}

fn start_receiver(port: u16, password: String, channel_sender: Sender<SenderAction>) {
	let addr = format!("0.0.0.0:{}", port);
	let socket_str = str::from_utf8(addr.as_bytes()).unwrap();
	let result = UdpSocket::bind(socket_str);
	let socket = match result {
			Err(_) => panic!("Couldnt create socket. {}", socket_str),
			Ok(v) => v
	};
	loop {
		let mut buffer = [0; RECEIVEBUFFERSIZE];
		let result = socket.recv_from(&mut buffer);
		let (no, _) = match result {
				Err(_) => continue,
				Ok((a,b)) => (a,b)
		};
		let data = Vec::from(&buffer[0..no]);
		let received_str = match str::from_utf8(&data) {
				Err(_) => { println!("Received non-utf8 data. Dropping it."); continue }, 
				Ok(v) => v
		};
		let msg = match extmsg::Message::from_string(received_str.to_string()) {
			Ok(v) => v,
			Err(_) => continue
		};
		//validate checksum before accepting the client
		if str::from_utf8(received_str.as_bytes()).unwrap() == str::from_utf8(password.as_bytes()).unwrap() {
			//println!("Password doesnt match");
			//checksum is invalid. silently disregard it
			continue;
		}
		match msg.action {
			extmsg::Action::Register => channel_sender.send(SenderAction::Register(addr.to_string())).unwrap(),
			extmsg::Action::Switch(d) => {
				switch(d);
				channel_sender.send(SenderAction::Update);
			}
		}
		
		thread::sleep_ms(INTERVAL);
	}
}

fn switch(data: extmsg::SwitchData) {
	println!("Switching device {} to {}", data.id, data.state.to_string());
	telldus::switch(data.id, data.state);
}
