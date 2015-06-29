use rustc_serialize::json::{self};
use std::str;
use telldus;

#[derive(RustcEncodable, RustcDecodable)]
pub enum Action {
	Register,
	Switch(SwitchData)
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Message {
	pub password: String,
	pub action: Action,
	pub data: String
}

impl Message {
	pub fn from_string(s: String) -> Result<Message, json::DecoderError> {
		return json::decode(str::from_utf8(s.as_bytes()).unwrap());
	}
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct SwitchData {
	pub id: i32,
	pub state: telldus::types::State
}
