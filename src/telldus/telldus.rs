use libc::{c_int, c_char};
use std::str;
use std::ffi::CStr;
use std::string::String;
use std::borrow::ToOwned;
use telldus::types;
use extmsg;

const STR_CAPACITY: i32 = 20;

#[allow(dead_code)]
enum DeviceMethod {
	TurnOn = 1,
	TurnOff = 2,
	Bell = 4,
	Toggle = 8,
	Dim = 16,
	Learn = 32,
	Execute = 64,
	Up = 128,
	Down = 256,
	Stop = 512
}

#[allow(dead_code)]
enum SensorValue {
	Temperature = 1,
	Humidity = 2,
	RainRate = 4,
	RainTotal = 8,
	WindDirection = 16,
	WindAverage = 32,
	WindGust = 64
}

#[allow(dead_code)]
enum ErrorCode {
	Success = 0,
	ErrorNotFound = -1,
	ErrorPermissionDenied = -2,
	ErrorDeviceNotFound = -3,
	ErrorMethodNotSupported = -4,
	ErrorCommunication = -5,
	ErrorConnectionService = -6,
	ErrorUnknownResponse = -7,
	ErrorSyntax = -8,
	ErrorBrokenPipe = -9,
	ErrorCommunicationService = -10,
	ErrorUnknown = -99
}

#[link(name = "telldus-core")]
#[allow(dead_code)]
extern {
	fn tdInit();
	fn tdClose();
	fn tdReleaseString(str: *const c_char);
	fn tdTurnOn(deviceId: c_int) -> c_int;
	fn tdTurnOff(deviceId: c_int) -> c_int;
	fn tdGetNumberOfDevices() -> c_int;
	fn tdLastSentCommand(id: c_int, methods: c_int) -> c_int;
	fn tdGetDeviceType(deviceId: c_int) -> c_int;
	fn tdGetDeviceId(deviceIndex: c_int) -> c_int;
	fn tdGetName(deviceId: c_int) -> *mut c_char;
	fn tdGetProtocol(deviceId: c_int) -> *mut c_char;
	fn tdGetModel(deviceId: c_int) -> *mut c_char;
	fn tdMethods(id: c_int, methods_supported: c_int) -> c_int;
	fn tdSensor(protocol: *const c_char, protocolLen: c_int, model: *const c_char, modelLen: c_int, id: *mut c_int, dataTypes: *mut c_int) -> c_int;
	fn tdSensorValue(protocol: *const c_char, model: *const c_char, id: c_int, dataType: c_int, value: *const c_char, len: c_int, timestamp: *mut c_int) -> c_int;
	fn tdController(id: *mut c_int, controller_type: *mut c_int, name: *const c_char, len: c_int, available: *mut c_int) -> c_int;
	fn tdControllerValue(id: c_int, name: *const c_char, value: *const c_char, len: c_int) -> c_int;
}

pub fn init() {
	unsafe {
		tdInit();
	}
}

pub fn close() {
	unsafe {
		tdClose();
	}
}

pub fn switch(id: i32, newstate: extmsg::State) {
	match newstate {
		extmsg::State::On => unsafe { tdTurnOn(id); },
		extmsg::State::Off => unsafe { tdTurnOff(id); }
	}
}

fn get_sensors() -> Vec<types::Sensor> {
	let mut sensors = Vec::new();
	unsafe {
		let mut return_val: c_int = 0;
		while return_val == ErrorCode::Success as i32 {
			let protocol_bytes = [0; STR_CAPACITY as usize];
			let model_bytes = [0; STR_CAPACITY as usize];
			
			let mut id: c_int = 0;
			let mut datatype: c_int = 0;
			
			return_val = tdSensor(protocol_bytes.as_ptr() as *const i8, STR_CAPACITY, model_bytes.as_ptr() as *const i8, STR_CAPACITY, &mut id, &mut datatype);
			let protocol = cchar_to_string(protocol_bytes.as_ptr() as *const i8).clone();
			let model = cchar_to_string(model_bytes.as_ptr() as *const i8).clone();

			if return_val == ErrorCode::Success as i32 {
				let mut timestamp = 0;
				let value_bytes = [0; STR_CAPACITY as usize];

				let return_sensor_value = tdSensorValue(protocol_bytes.as_ptr() as *const i8, model_bytes.as_ptr() as *const i8, id, SensorValue::Temperature as i32, value_bytes.as_ptr() as *const i8, STR_CAPACITY, &mut timestamp);

				//skip devices which fail getting temperature
				if return_sensor_value != ErrorCode::Success as i32 { continue; }

				let value_string = cchar_to_string(value_bytes.as_ptr() as *const i8);
				let temperature = value_string.parse::<f32>().unwrap();

				let sensor = types::Sensor {id: id,
								protocol: protocol,
								model: model,
								datatypes: datatype, 
								temperature: temperature,
								timestamp: timestamp};
				//println!("{}", sensor.to_string());

				sensors.push(sensor);
			}
		}
		return sensors;
	}
}

fn map_state(value: i32) -> extmsg::State {
	if value == DeviceMethod::TurnOn as i32 {
		return extmsg::State::On;
	} else {
		return extmsg::State::Off;
	}
}

fn get_devices() -> Vec<types::Device> {
	let mut devices = Vec::new();
	unsafe {
		let num_devices = tdGetNumberOfDevices();
		for i in 0..num_devices {
			//get id
			let id = tdGetDeviceId(i);

			//is it a on/off-device?
			let methods = tdMethods(id, DeviceMethod::TurnOn as i32 | DeviceMethod::TurnOff as i32);
			//ignore if not
			if methods < ErrorCode::Success as i32 {
				println!("Device with id {} is not a turn on/off device. Skipping. Method code: {}", id, methods);
				continue;
			}
			//only allow on/off devices
			if !is_on_off_device(methods) {
				println!("Device with id {} is not a turn on/off device. Skipping. Method code: {}", id, methods);
				continue;
			}

			//get device name
			let name_ptr = tdGetName(id);
			let name = cchar_to_string(name_ptr).clone();
			tdReleaseString(name_ptr);
			
			//get last sent command. which is the only status that is :(
			let lastsent = tdLastSentCommand(id, methods);
			if lastsent < ErrorCode::Success as i32 {
				println!("tdLastSentCommand failed. Returned value {}", lastsent);
				continue;
			}
			let state = map_state(lastsent);
			
			let device = types::Device { id: id, name: name, state: state };
			devices.push(device);
		}
	}
	return devices;
}

fn is_on_off_device(methods: i32) -> bool {
	let req = DeviceMethod::TurnOn as i32 | DeviceMethod::TurnOff as i32;
	let mask = req & methods;
	return mask != 0;
}

pub fn get_status() -> types::Status {
	return types::Status{ sensors: get_sensors(), devices: get_devices() };
}

fn cchar_to_string(char_ptr: *const c_char) -> String {
	unsafe {
		return str::from_utf8(CStr::from_ptr(char_ptr).to_bytes()).unwrap().to_owned();
	}
}
