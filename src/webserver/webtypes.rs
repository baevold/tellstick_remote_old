use rustc_serialize::json::{self};
use std::str;
use common::extmsg;

#[derive(RustcEncodable, RustcDecodable)]
pub enum Action {
	Login,
	RequestStatus,
	Status(Status),
	SetTemp(ZoneTemp)
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Message {
	pub hash: String,
	pub action: Action
}

impl Message {
        pub fn from_string(s: String) -> Result<Message, json::DecoderError> {
                return json::decode(str::from_utf8(s.as_bytes()).unwrap());
        }
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct Status {
	pub zones: Vec<Zone>
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct Zone {
	pub name: String,
	pub temp: f32,
	pub target: f32,
	pub switches: Vec<Switch>
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct Switch {
	pub name: String,
	pub state: extmsg::State
}	

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct ZoneTemp {
	pub name: String,
	pub temp: f32
}
