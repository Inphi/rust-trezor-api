use super::Model;
use crate::protos::MessageType;

pub mod error;
#[cfg(feature = "hid-device")]
pub mod hid;
pub mod protocol;
pub mod webusb;

/// A protobuf message accompanied by the message type.  This type is used to pass messages over the
/// transport and used to contain messages received from the transport.
pub struct ProtoMessage(pub MessageType, pub Vec<u8>);

impl ProtoMessage {
	pub fn new(mt: MessageType, payload: Vec<u8>) -> ProtoMessage {
		ProtoMessage(mt, payload)
	}
	pub fn message_type(&self) -> MessageType {
		self.0
	}
	pub fn payload(&self) -> &[u8] {
		&self.1
	}
	pub fn into_payload(self) -> Vec<u8> {
		self.1
	}

	/// Take the payload from the ProtoMessage and parse it to a protobuf message.
	pub fn into_message<M: protobuf::Message>(self) -> Result<M, protobuf::error::ProtobufError> {
		Ok(protobuf::Message::parse_from_bytes(&self.into_payload())?)
	}
}

/// The transport interface that is implemented by the different ways to communicate with a Trezor
/// device.
pub trait Transport {
	fn session_begin(&mut self) -> Result<(), error::Error>;
	fn session_end(&mut self) -> Result<(), error::Error>;

	fn write_message(&mut self, message: ProtoMessage) -> Result<(), error::Error>;
	fn read_message(&mut self) -> Result<ProtoMessage, error::Error>;
}

mod constants {
	//! A collection of transport-global constants.

	pub const DEV_TREZOR1: (u16, u16) = (0x534C, 0x0001);
	pub const DEV_TREZOR2: (u16, u16) = (0x1209, 0x53C1);
	pub const DEV_TREZOR2_BL: (u16, u16) = (0x1209, 0x53C0);
}

/// Derive the Trezor model from the HID device.
pub(crate) fn derive_model(dev_id: (u16, u16)) -> Option<Model> {
	match dev_id {
		constants::DEV_TREZOR1 => Some(Model::Trezor1),
		constants::DEV_TREZOR2 => Some(Model::Trezor2),
		constants::DEV_TREZOR2_BL => Some(Model::Trezor2Bl),
		_ => None,
	}
}
