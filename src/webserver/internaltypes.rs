use std::sync::mpsc::Sender;
use common::telldus_types;

use webserver::webtypes;

pub enum InternalAction {
	RequestStatus(Sender<telldus_types::Status>),
	Status(webtypes::Status),
	TellstickStatus(telldus_types::Status)
}
