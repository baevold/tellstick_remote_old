use rustc_serialize::json::{self};

#[derive(RustcEncodable, RustcDecodable)]
pub enum Action {
	Login(String)
}
