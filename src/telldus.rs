
use libc::c_int;

#[link(name = "telldus-core")]
extern {
    fn tdGetNumberOfDevices() -> c_int;
}

pub fn get_number_of_devices() -> i32 {
    unsafe {
        return tdGetNumberOfDevices();
    }
}
