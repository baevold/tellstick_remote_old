use std::sync::mpsc::Sender;
use common::telldus_types;

use webserver::webtypes;

pub enum InternalAction {
	RequestStatus(Sender<webtypes::Status>),
	TellstickStatus(telldus_types::Status),
	SetTemp(webtypes::ZoneTemp)
}
