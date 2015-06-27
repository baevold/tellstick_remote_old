use std::thread;
use std::sync::mpsc::Sender;

static INTERVAL: u32 = 3000;

pub fn start(channel_sender: Sender<String>) -> thread::JoinHandle<()> {
	return thread::spawn(move || { start_receiver(channel_sender); });
}

fn start_receiver(channel_sender: Sender<String>) {
	loop {
		channel_sender.send(String::from("yo")).unwrap();
		thread::sleep_ms(INTERVAL);
	}
}
