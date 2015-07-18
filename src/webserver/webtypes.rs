use rustc_serialize::json::{self};
use std::str;

#[derive(RustcEncodable, RustcDecodable)]
pub enum Action {
	Login(String)
}

impl Action {
        pub fn from_string(s: String) -> Result<Action, json::DecoderError> {
                return json::decode(str::from_utf8(s.as_bytes()).unwrap());
        }
}
