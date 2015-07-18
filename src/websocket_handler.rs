mod webserver;

extern crate websocket;
extern crate rustc_serialize;

fn main() {
	webserver::main();
	println!("hello");
}
