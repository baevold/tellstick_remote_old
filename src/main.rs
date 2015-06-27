mod main_test;
mod telldus;

extern crate libc;

pub mod main {
    pub fn retvalue() -> i32 {
        return 2;
    }
}

#[allow(dead_code)]
fn main() {
    println!("Hello, world! {0}", main::retvalue());
    //telldus::get_devices();
    telldus::get_sensors();
}


#[test]
fn test_retvalue() {
    assert!(main::retvalue()==2);
}
