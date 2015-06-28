use std::thread;
use std::sync::mpsc::Sender;
use std::net::UdpSocket;
use std::str;

const INTERVAL: u32 = 3000;
const RECEIVEBUFFERSIZE: usize = 1024;

pub fn start(port: u16, password: String, channel_sender: Sender<String>) -> thread::JoinHandle<()> {
	return thread::spawn(move || { start_receiver(port, password, channel_sender); });
}

fn start_receiver(port: u16, password: String, channel_sender: Sender<String>) {
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
		let (no_of_bytes, addr) = match result {
				Err(_) => continue,
				Ok((a,b)) => (a,b)
		};
		println!("{}", addr.to_string());
		let received_str = match str::from_utf8(&buffer) {
				Err(_) => { println!("Received non-utf8 data. Dropping it."); continue }, 
				Ok(v) => v
		};
		//validate checksum before accepting the client
		if str::from_utf8(received_str.as_bytes()).unwrap() == str::from_utf8(password.as_bytes()).unwrap() {
			println!("Password doesnt match");
			//checksum is invalid. silently disregard it
			continue;
		}
		println!("Adding client {}", addr.to_string());
		channel_sender.send(addr.to_string()).unwrap();
		
		thread::sleep_ms(INTERVAL);
	}
}
