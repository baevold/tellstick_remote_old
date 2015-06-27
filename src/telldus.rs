
#![feature(libc)]
use libc::{c_int, c_char};
use std::str;
use std::ffi::CStr;
use std::ffi::CString;
use std::string::String;
use std::ptr;
use std::borrow::ToOwned;


#[allow(dead_code)]
enum DEVICEMETHOD {
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

#[allow(dead_code)]
enum DEVICETYPE {
    DEVICE = 1,
    GROUP = 2,
    SCENE = 3
}

#[allow(dead_code)]
enum SENSORVALUE {
    TEMPERATURE = 1,
    HUMIDITY = 2,
    RAINRATE = 4,
    RAINTOTAL = 8,
    WINDDIRECTION = 16,
    WINDAVERAGE = 32,
    WINDGUST = 64
}

#[allow(dead_code)]
enum ERRORCODE {
    SUCCESS = 0,
    ERRORNOTFOUND = -1,
    ERRORPERMISSIONDENIED = -2,
    ERRORDEVICENOTFOUND = -3,
    ERRORMETHODNOTSUPPORTED = -4,
    ERRORCOMMUNICATION = -5,
    ERRORCONNECTINGSERVICE = -6,
    ERRORUNKNOWNRESPONSE = -7,
    ERRORSYNTAX = -8,
    ERRORBROKENPIPE = -9,
    ERRORCOMMUNICATIONSERVICE = -10,
    ERRORUNKNOWN = -99
}

#[derive(Clone)]
pub struct Sensor {
    pub id: i32,
    pub protocol: String,
    pub model: String,
    pub datatypes: i32,
    pub temperature: f32
}

impl Sensor {
    fn to_string(&self) -> String {
        return format!("Sensor: protocol={} model={} id={} datatypes={} temperature={}",self.protocol, self.model, self.id, self.datatypes, self.temperature);
    }
}

#[link(name = "telldus-core")]
#[allow(dead_code)]
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
    fn tdMethods(id: c_int, methods_supported: c_int) -> c_int;
    fn tdSensor(protocol: *const c_char, protocolLen: c_int, model: *const c_char, modelLen: c_int, id: *mut c_int, dataTypes: *mut c_int) -> c_int;
    fn tdSensorValue(protocol: *const c_char, model: *const c_char, id: c_int, dataType: c_int, value: *const c_char, len: c_int, timestamp: *mut c_int) -> c_int;
}

/*
pub fn get_sensors() -> Vec<Sensor> {
    let sensors = get_sensor_structs();
    for mut sensor in sensors {
        let val_string = get_sensor_value(sensor.clone());
        sensor.temperature = val_string.parse::<f32>().unwrap();
        println!("{}", sensor.to_string());
    }
    return sensors();
}
*/

fn get_sensor_value(sensor: Sensor) -> String {
    unsafe {
        let protocol_cstr = CString::new(sensor.protocol).unwrap();
        let protocol_bytes = protocol_cstr.as_bytes_with_nul();
        let model_cstr = CString::new(sensor.model).unwrap();
        let model_bytes = model_cstr.as_bytes_with_nul();
        let str_capacity = 20;
        let mut timestamp = 0;
        let value_cstr = CString::new(String::with_capacity(str_capacity as usize)).unwrap();
        let value_bytes = value_cstr.as_bytes_with_nul();
        let return_sensor_value = tdSensorValue(protocol_bytes.as_ptr() as *const i8, model_bytes.as_ptr() as *const i8, sensor.id, SENSORVALUE::TEMPERATURE as i32, value_bytes.as_ptr() as *const i8, str_capacity, &mut timestamp);
        if return_sensor_value == ERRORCODE::SUCCESS as i32 {
            return cchar_to_string(value_bytes.as_ptr() as *const i8);
        } else {
            return String::from_str("0.0");
        }
    }
}

pub fn get_sensors() -> Vec<Sensor> {
    let mut sensors = Vec::new();
    unsafe {
        let mut return_val: c_int = 0;
        while(return_val == ERRORCODE::SUCCESS as i32) {
            let str_capacity: i32 = 20;
            let prot_cstr = CString::new(String::with_capacity(str_capacity as usize)).unwrap();
            let prot_bytes = prot_cstr.as_bytes_with_nul();
            let protocol_ptr = prot_bytes.as_ptr() as *const i8;
            let model_cstr = CString::new(String::with_capacity(str_capacity as usize)).unwrap();
            let model_bytes = model_cstr.as_bytes_with_nul();
            let model_ptr = model_bytes.as_ptr() as *const i8;
            
            let mut id: c_int = 0;
            let mut datatype: c_int = 0;
            
            return_val = tdSensor(protocol_ptr, str_capacity, model_ptr, str_capacity, &mut id, &mut datatype);
            
            if(return_val == ERRORCODE::SUCCESS as i32) {
                let protocol = cchar_to_string(protocol_ptr).clone();
                let model = cchar_to_string(model_ptr).clone();
                let protocol_cstr = CString::new(protocol.clone()).unwrap();
                let protocol_bytes = protocol_cstr.as_bytes_with_nul();
                let model_cstr = CString::new(model.clone()).unwrap();
                let model_bytes = model_cstr.as_bytes_with_nul();

                let mut timestamp = 0;
                let value_cstr = CString::new(String::with_capacity(str_capacity as usize)).unwrap();
                let value_bytes = value_cstr.as_bytes_with_nul();

                let return_sensor_value = tdSensorValue(protocol_bytes.as_ptr() as *const i8, model_bytes.as_ptr() as *const i8, id, SENSORVALUE::TEMPERATURE as i32, value_bytes.as_ptr() as *const i8, str_capacity, &mut timestamp);
                let mut temperature = 0.0;
                if return_sensor_value == ERRORCODE::SUCCESS as i32 {
                    let value_string = cchar_to_string(value_bytes.as_ptr() as *const i8);
                    temperature = value_string.parse::<f32>().unwrap();
                }

                let sensor: Sensor = Sensor {id: id,
                                             protocol: protocol.clone(),
                                             model: model.clone(),
                                             datatypes: datatype, 
                                             temperature: temperature};
                println!("{}", sensor.to_string());

                sensors.push(sensor);
            }
        }
        return sensors;
    }
}

pub fn get_devices() {
    unsafe {
        let num_devices = tdGetNumberOfDevices();
        for i in 0..num_devices {
            let mut id = tdGetDeviceId(i);
            let protocol = cchar_to_string(tdGetProtocol(id));
            let model = cchar_to_string(tdGetModel(id));
            println!("id: {} protocol: {} model: {}", id, protocol, model);

            let str_capacity: i32 = 16;
            let str_capacity_cint = str_capacity as c_int;

            let empty_value = String::with_capacity(str_capacity as usize);
            let cstr_value = CString::new(empty_value.into_bytes()).unwrap().as_ptr();
            let mut timestamp: i32 = 0;

            let success = tdSensorValue(tdGetProtocol(id), tdGetModel(id), id, 1, cstr_value, str_capacity, &mut timestamp);
            //let value = cchar_to_string(cstr_value);
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
        }
    }
}

fn cchar_to_string(char_ptr: *const c_char) -> String {
    unsafe {
        return str::from_utf8(CStr::from_ptr(char_ptr).to_bytes()).unwrap().to_owned();
    }
}
