pub mod types;
mod telldus;

pub fn init() {
	telldus::init();
}

pub fn close() {
	telldus::close();
}

pub fn get_status() -> types::Status {
	return telldus::get_status();
}
