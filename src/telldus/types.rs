use std::string::String;

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct Sensor {
	pub id: i32,
	pub protocol: String,
	pub model: String,
	pub datatypes: i32,
	pub temperature: f32,
	pub timestamp: i32
}

impl ToString for Sensor {
	#[allow(dead_code)]
	fn to_string(&self) -> String {
		return format!("Sensor: protocol={} model={} id={} datatypes={} temperature={} timestamp={}",self.protocol, self.model, self.id, self.datatypes, self.temperature, self.timestamp);
	}
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Status {
	pub sensors: Vec<Sensor>,
	pub devices: Vec<Device>
}


impl ToString for Status {
	#[allow(dead_code)]
	fn to_string(&self) -> String {
		fn sensors_to_string(vector: &Vec<Sensor>) -> String {
			let mut tmp = String::from("[ ");
			for v in vector {
				tmp = format!("{} ", v.to_string());
			}
			tmp = format!("{}{}", tmp, String::from(" ]"));
			return tmp;
		}
		fn devices_to_string(vector: &Vec<Device>) -> String {
			let mut tmp = String::from("[ ");
			for v in vector {
				tmp = format!("{} ", v.to_string());
			}
			tmp = format!("{}{}", tmp, String::from(" ]"));
			return tmp;
		}
		return format!("Status: {} {}", sensors_to_string(&self.sensors), devices_to_string(&self.devices));
	}
}

#[derive(RustcEncodable, RustcDecodable)]
pub enum State {
	On,
	Off
}

impl ToString for State {
	fn to_string(&self) -> String {
		let ret = match *self {
			State::On  => "ON",
			State::Off => "OFF"
		};
		return String::from(ret);
	}
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Device {
	pub id: i32,
	pub name: String,
	pub state: State
}

impl ToString for Device {
	#[allow(dead_code)]
	fn to_string(&self) -> String {
		return format!("Device: id={} name={} state={}", self.id, self.name, self.state.to_string());
	}
}
