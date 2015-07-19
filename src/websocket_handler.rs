mod webserver;
mod common;

extern crate websocket;
extern crate rustc_serialize;

fn main() {
	webserver::main();
	println!("hello");
}
