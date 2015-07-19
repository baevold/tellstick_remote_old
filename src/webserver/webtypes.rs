use rustc_serialize::json::{self};
use std::str;

#[derive(RustcEncodable, RustcDecodable)]
pub enum Action {
	Login,
	RequestStatus
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
