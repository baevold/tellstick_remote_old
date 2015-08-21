mod webserver;
mod common;

extern crate websocket;
extern crate rustc_serialize;

#[macro_use]
extern crate log;
extern crate env_logger;

fn main() {
	env_logger::init().unwrap();
	webserver::main();
}
