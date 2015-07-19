use rustc_serialize::json::{self};
use std::str;

#[derive(RustcEncodable, RustcDecodable, Clone)]
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
pub struct SwitchData {
	pub id: i32,
	pub state: State
}

#[derive(RustcEncodable, RustcDecodable)]
pub enum Action {
	Register,
	Switch(SwitchData)
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Message {
	pub password: String,
	pub action: Action,
}

#[allow(dead_code)]
impl Message {
	pub fn from_string(s: String) -> Result<Message, json::DecoderError> {
		return json::decode(str::from_utf8(s.as_bytes()).unwrap());
	}
}

#[allow(dead_code)]
pub fn write_message() {
	let m = Message {
			password: String::from("pwd"),
			action: Action::Switch(SwitchData { id: 1, state: State::On }),
	};
	let data: String = json::encode(&m).unwrap();
        println!("use echo '[data]' > jq . to prettyfi. Remember to quites!");
	println!("{}", data);
}
