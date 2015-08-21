mod webtypes;
mod config;
mod wshandler;
mod internaltypes;
mod statusreceiver;

use std::thread;
use std::sync::Arc;
use std::sync::mpsc;
use std::str;
use rustc_serialize::json::{self};
use common::telldus_types;
use common::extmsg;
use std::net::UdpSocket;


pub fn main() {
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

	//need config here as well
	let config_main = config_arc.clone();

	//let this thread handle interthread communication and matching between telldus status and web status
	//init a telldus status
	let sensor = telldus_types::Sensor{ id: 1, protocol: "dummy".to_string(), model: "dummy".to_string(), datatypes: 1, temperature: 10.0, timestamp: 0 };
	let device = telldus_types::Device{ id: 1, name: "dummyswitch".to_string(), state: extmsg::State::Off };
	let mut sensors: Vec<telldus_types::Sensor> = Vec::new();
	sensors.push(sensor);
	let mut devices: Vec<telldus_types::Device> = Vec::new();
	devices.push(device);
	let mut telldus_status = telldus_types::Status{sensors: sensors, devices: devices};
	
	//read mapping
	config::write_mapping();
	let mut mapping = config::read_mapping().unwrap();

	//get initial web status
	let mut wsclients = Vec::new();
	let mut webstatus = to_webstatus(&telldus_status, &mapping);
	loop {
		let action = rx.recv().unwrap();
		let wss = webstatus.clone();
		match action {
			internaltypes::InternalAction::RequestStatus(tx) => { tx.send(wss).unwrap(); },
			internaltypes::InternalAction::TellstickStatus(status) => {
				telldus_status = status;
				webstatus = to_webstatus(&telldus_status, &mapping);
				wsclients = update_clients(wsclients, &webstatus);
				update_switches(&mapping, &telldus_status, &config_main.telldus_client, &config_main.telldus_password);
			}
			internaltypes::InternalAction::SetTemp(zonetemp) => {
				set_new_temp(&mut mapping, zonetemp);
				//update_switches(&mapping, &telldus_status, &config_main.telldus_client, &config_main.telldus_password);
			}
			internaltypes::InternalAction::AddClient(tx) => {
				wsclients.push(tx);
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
			let state = match get_switch_state(&status, switch.id) {
				Some(s) => s,
				None => { return webtypes::Status{zones: zones}; }
			};
			let switch = webtypes::Switch{ name: switch.name, state: state };
			webswitches.push(switch);
		}
		let temp = match get_temp(&status, zone.id) {
			Some(t) => t,
			None => 0.0
		};
		let webzone = webtypes::Zone{ name: zone.name, temp: temp, target: zone.target, switches: webswitches };
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
	println!("Could not find switch with id {}", id);
	return None;
}

fn get_temp(status: &telldus_types::Status, id: i32) -> Option<f32> {
	let status = status.clone();
	for sensor in status.sensors {
		if sensor.id == id {
			return Some(sensor.temperature);
		}
	}
	println!("Could not find sensor with id {}", id);
	return None;
}

fn set_new_temp(oldmapping: &mut config::Mapping, zonetemp: webtypes::ZoneTemp) {
	for zone in &mut oldmapping.zones {
		if str::from_utf8(zone.name.as_bytes()).unwrap() == str::from_utf8(zonetemp.name.as_bytes()).unwrap() {
			zone.target = zonetemp.temp;
		}
	}
}

fn update_switches(mapping: &config::Mapping, status: &telldus_types::Status, client: &String, password: &String) {
	fn get_sensor_by_id(id: i32, sensors: &Vec<telldus_types::Sensor>) -> Option<telldus_types::Sensor> {
		let sensors = sensors.clone();
		for sensor in sensors {
			if sensor.id == id { return Some(sensor); }
		}
		return None;
	}
	fn get_device_by_id(id: i32, devices: &Vec<telldus_types::Device>) -> Option<telldus_types::Device> {
		let devices = devices.clone();
		for device in devices {
			if device.id == id { return Some(device); }
		}
		return None;
	}
	let mut switch_list = Vec::new();
	let zones = mapping.zones.clone();
	for zone in zones {
		let sensor = match get_sensor_by_id(zone.id, &status.sensors) {
			Some(s) => s,
			None => { error!("Could not get sensor for zone.id={}", zone.id); return; }
		};
		if sensor.temperature < zone.target {
			let switches = zone.switches.clone();
			for switch in switches {
				let switch = match get_device_by_id(switch.id, &status.devices) {
					Some(s) => s,
					None => { error!("Could not get device with id={}", switch.id); return; }
				};
				match switch.state {
					extmsg::State::Off => {
						let sd = extmsg::SwitchData { id: switch.id, state: extmsg::State::On };
						switch_list.push(sd);
					},
					extmsg::State::On => ()
				}
			}
		}
		if sensor.temperature > zone.target {
			let switches = zone.switches.clone();
			for switch in switches {
				let switch = get_device_by_id(switch.id, &status.devices).unwrap();
				match switch.state {
					extmsg::State::Off => (),
					extmsg::State::On => {
						let sd = extmsg::SwitchData { id: switch.id, state: extmsg::State::Off };
						switch_list.push(sd);
					}
				}
			}
		}
	}
	switch_devices(switch_list, client, password);
}

fn switch_devices(swlist: Vec<extmsg::SwitchData>, client: &String, password: &String) {
	if swlist.len() == 0 { return; }
	let msg = extmsg::Message { password: password.clone(), action: extmsg::Action::Switch(swlist) };
	let vec = client.split(":").collect::<Vec<&str>>();
	let ip = vec[0];
	let port = String::from(vec[1]).parse::<u16>().unwrap();
	let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
	let data: String = json::encode(&msg).unwrap();
	let buf = data.into_bytes();
	socket.send_to(&buf, (ip, port)).unwrap();
	drop(socket);
}

fn update_clients(clients: Vec<mpsc::Sender<internaltypes::WebsocketSendAction>>, webstatus: &webtypes::Status) -> Vec<mpsc::Sender<internaltypes::WebsocketSendAction>> {
	let mut newclients = Vec::new();
	let status = json::encode(&webstatus).unwrap();
	for client in clients {
		match client.send(internaltypes::WebsocketSendAction::Message(status.clone())) {
			Ok(_) => { newclients.push(client)},
			Err(_) => {}
		}
	}
	return newclients;
}

