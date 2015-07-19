use std::thread;
use std::sync::Arc;
use std::sync::mpsc;
use rustc_serialize::json::{self};
use common::telldus_types;
use common::extmsg;

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

	//let this thread handle interthread communication and matching between telldus status and web status
	//init a telldus status
	let sensors: Vec<telldus_types::Sensor> = Vec::new();
	let devices: Vec<telldus_types::Device> = Vec::new();
	let mut telldus_status = telldus_types::Status{sensors: sensors, devices: devices};
	
	//read mapping
	let mapping = config::read_mapping().unwrap();

	//get initial web status
	let mut webstatus = to_webstatus(&telldus_status, &mapping);
	loop {
		let action = rx.recv().unwrap();
		let wss = webstatus.clone();
		match action {
			internaltypes::InternalAction::RequestStatus(tx) => { tx.send(wss).unwrap(); },
			internaltypes::InternalAction::TellstickStatus(status) => {
				telldus_status = status;
				webstatus = to_webstatus(&telldus_status, &mapping);
			}
		};
	}
}

fn to_webstatus(status: &telldus_types::Status, mapping: &config::Mapping) -> webtypes::Status {
	let mut zones = Vec::new();
	let mapping = mapping.clone();
	for zone in mapping.zones {
		let mut webswitches = Vec::new();
		for switch in zone.switches {
			let state = get_switch_state(&status, switch.id).unwrap();
			let switch = webtypes::Switch{ name: switch.name, state: state };
			webswitches.push(switch);
		}
		let temp = match get_temp(&status, zone.id) {
			Some(t) => t,
			None => 0.0
		};
		let webzone = webtypes::Zone{ name: zone.name, temp: temp, switches: webswitches };
		zones.push(webzone);
	}
	let webstatus = webtypes::Status{zones: zones};
	return webstatus;
}

fn get_switch_state(status: &telldus_types::Status, id: i32) -> Option<extmsg::State> {
	let status = status.clone();
	for device in status.devices {
		if device.id == id {
			return Some(device.state);
		}
	}
	return None;
}

fn get_temp(status: &telldus_types::Status, id: i32) -> Option<f32> {
	let status = status.clone();
	for sensor in status.sensors {
		if sensor.id == id {
			return Some(sensor.temperature);
		}
	}
	return None;
}

