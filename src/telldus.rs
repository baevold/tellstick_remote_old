

use libc::{c_int, c_char};
use std::str;
use std::ffi::CStr;
use std::ffi::CString;
use std::string::String;
use std::ptr;
use std::borrow::ToOwned;

enum DEVICE_METHOD {
    TURNON = 1,
    TURNOFF = 2,
    BELL = 4,
    TOGGLE = 8,
    DIM = 16,
    LEARN = 32,
    EXECUTE = 64,
    UP = 128,
    DOWN = 256,
    STOP = 512
}

enum DEVICE_TYPE {
    DEVICE = 1,
    GROUP = 2,
    SCENE = 3
}

enum SENSOR_VALUE {
    TEMPERATURE = 1,
    HUMIDITY = 2,
    RAINRATE = 4,
    RAINTOTAL = 8,
    WINDDIRECTION = 16,
    WINDAVERAGE = 32,
    WINDGUST = 64
}

#[link(name = "telldus-core")]
extern {
    fn tdInit();
    fn tdClose();
    fn tdTurnOn(deviceId: c_int) -> c_int;
    fn tdTurnOff(deviceId: c_int) -> c_int;
    fn tdGetNumberOfDevices() -> c_int;
    fn tdGetDeviceType(deviceId: c_int) -> c_int;
    fn tdGetDeviceId(deviceIndex: c_int) -> c_int;
    fn tdGetName(deviceId: c_int) -> *mut c_char;
    fn tdGetProtocol(deviceId: c_int) -> *mut c_char;
    fn tdGetModel(deviceId: c_int) -> *mut c_char;
    fn tdSensor(protocol: *const c_char, protocolLen: c_int, model: *const c_char, modelLen: c_int, id: *mut c_int, dataTypes: *mut c_int) -> c_int;
    fn tdSensorValue(protocol: *const c_char, model: *const c_char, id: c_int, dataType: c_int, value: *const c_char, len: c_int, timestamp: *mut c_int) -> c_int;
}

pub fn get_number_of_devices() -> i32 {
    unsafe {
        return tdGetNumberOfDevices();
    }
}

pub fn get_sensors() {
    unsafe {
        let num_devices = tdGetNumberOfDevices();
        for i in 0..num_devices {
            let mut id = tdGetDeviceId(i);
            let protocol = cchar_to_str(tdGetProtocol(id));
            let model = cchar_to_str(tdGetModel(id));
            println!("id: {} protocol: {} model: {}", id, protocol, model);

            let str_capacity: i32 = 16;
            let str_capacity_cint = str_capacity as c_int;

            let mut empty_value = String::with_capacity(str_capacity as usize);
            let mut cstr_value = CString::new(empty_value.into_bytes()).unwrap().as_ptr();
            let mut timestamp: i32 = 0;

            let success = tdSensorValue(tdGetProtocol(id), tdGetModel(id), id, 1, cstr_value, str_capacity, &mut timestamp);
            //let value = cchar_to_str(cstr_value);
            //println!("value retrieved: {}", value);
            println!("success={}",success);
            
            //let cstring_protocol = CStr::new("            ").unwrap();

            let empty_protocol = String::with_capacity(str_capacity as usize);
            let cstr_protocol = CString::new(empty_protocol.into_bytes()).unwrap();

            let empty_model = String::with_capacity(str_capacity as usize);
            let cstr_model = CString::new(empty_model.into_bytes()).unwrap().as_ptr();

            //let mut id: i32 = 0;
            let id_ptr: *mut i32 = &mut id;

            let mut datatype: i32 = 0;
            let datatype_ptr: *mut i32 = &mut datatype; 

            let ret = tdSensor(cstr_protocol.as_ptr(), str_capacity_cint, cstr_model, str_capacity_cint, id_ptr, datatype_ptr);
            println!("tdSensor returned {}", ret);
        }
    }
}

fn cchar_to_str(char_ptr: *mut c_char) -> String {
    unsafe {
        return str::from_utf8(CStr::from_ptr(char_ptr).to_bytes()).unwrap().to_owned();
    }
}
