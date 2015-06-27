use libc::{c_int, c_char};
use std::str;
use std::ffi::CStr;
use std::ffi::CString;
use std::string::String;
use std::borrow::ToOwned;

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

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct Sensor {
	pub id: i32,
	pub protocol: String,
	pub model: String,
	pub datatypes: i32,
	pub temperature: f32,
	pub timestamp: i32
}

impl Sensor {
	#[allow(dead_code)]
	pub fn to_string(&self) -> String {
		return format!("Sensor: protocol={} model={} id={} datatypes={} temperature={} timestamp={}",self.protocol, self.model, self.id, self.datatypes, self.temperature, self.timestamp);
	}
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Status {
	pub sensors: Vec<Sensor>,
	pub devices: Vec<Device>
}

#[derive(RustcEncodable, RustcDecodable)]
pub enum State {
	On,
	Off
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Device {
	pub id: i32,
	pub name: String,
	pub state: State
}

impl Device {
	fn state_to_string(&self, state: &State) -> String {
		let ret = match *state {
			State::On  => "ON",
			State::Off => "OFF"
		};
		return String::from(ret);
	}

	#[allow(dead_code)]
	pub fn to_string(&self) -> String {
		return format!("Device: id={} name={} state={}", self.id, self.name, self.state_to_string(&self.state));
	}
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

fn get_sensors() -> Vec<Sensor> {
	let mut sensors = Vec::new();
	unsafe {
		let mut return_val: c_int = 0;
		while return_val == ErrorCode::Success as i32 {
			let str_capacity: i32 = 20;
			let protocol_cstr = CString::new(String::with_capacity(str_capacity as usize)).unwrap();
			let protocol_bytes = protocol_cstr.as_bytes_with_nul();
			let model_cstr = CString::new(String::with_capacity(str_capacity as usize)).unwrap();
			let model_bytes = model_cstr.as_bytes_with_nul();
			
			let mut id: c_int = 0;
			let mut datatype: c_int = 0;
			
			return_val = tdSensor(protocol_bytes.as_ptr() as *const i8, str_capacity, model_bytes.as_ptr() as *const i8, str_capacity, &mut id, &mut datatype);
			
			if return_val == ErrorCode::Success as i32 {
				let protocol = cchar_to_string(protocol_bytes.as_ptr() as *const i8).clone();
				let model = cchar_to_string(model_bytes.as_ptr() as *const i8).clone();
				let protocol_cstr = CString::new(protocol.clone()).unwrap();
				let protocol_bytes = protocol_cstr.as_bytes_with_nul();
				let model_cstr = CString::new(model.clone()).unwrap();
				let model_bytes = model_cstr.as_bytes_with_nul();

				let mut timestamp = 0;
				let value_cstr = CString::new(String::with_capacity(str_capacity as usize)).unwrap();
				let value_bytes = value_cstr.as_bytes_with_nul();

				let return_sensor_value = tdSensorValue(protocol_bytes.as_ptr() as *const i8, model_bytes.as_ptr() as *const i8, id, SensorValue::Temperature as i32, value_bytes.as_ptr() as *const i8, str_capacity, &mut timestamp);

				//skip devices which fail getting temperature
				if return_sensor_value != ErrorCode::Success as i32 { continue; }

				let value_string = cchar_to_string(value_bytes.as_ptr() as *const i8);
				let temperature = value_string.parse::<f32>().unwrap();

				let sensor: Sensor = Sensor {id: id,
											 protocol: protocol.clone(),
											 model: model.clone(),
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

fn map_state(value: i32) -> State {
	if value == DeviceMethod::TurnOn as i32 {
		return State::On;
	} else {
		return State::Off;
	}
}

fn get_devices() -> Vec<Device> {
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
				println!("{}", methods);
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
			//only allow on/off devices
			if !(lastsent == DeviceMethod::TurnOn as i32 && lastsent == DeviceMethod::TurnOff as i32) {
				continue;
			}
			let state = map_state(lastsent);
			
			let device = Device { id: id, name: name, state: state };
			devices.push(device);
		}
	}
	return devices;
}

pub fn get_status() -> Status {
	return Status{ sensors: get_sensors(), devices: get_devices() };
}

fn cchar_to_string(char_ptr: *const c_char) -> String {
	unsafe {
		return str::from_utf8(CStr::from_ptr(char_ptr).to_bytes()).unwrap().to_owned();
	}
}
