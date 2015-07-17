mod report;
mod common;

extern crate libc;
extern crate rustc_serialize;
extern crate time;

fn main() {
	report::main();
}
