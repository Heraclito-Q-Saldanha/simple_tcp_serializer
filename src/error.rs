use std::{
	fmt,
	io::{
		self,
		ErrorKind
	}
};

#[derive(Debug, Clone)]
pub enum NetworkPeerError{
	Parse,
	WouldBlock,
	NotConnected,
	ConnectionAborted,
	ConnectionReset,
	Unknown
}

impl fmt::Display for NetworkPeerError{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl From<bincode::Error> for NetworkPeerError{
	fn from(_err: bincode::Error) -> NetworkPeerError{
		NetworkPeerError::Parse
	}
}

impl From<io::Error> for NetworkPeerError{
	fn from(err: io::Error) -> NetworkPeerError{
		match err.kind(){
			ErrorKind::WouldBlock => NetworkPeerError::WouldBlock,
			ErrorKind::NotConnected => NetworkPeerError::NotConnected,
			ErrorKind::ConnectionAborted => NetworkPeerError::ConnectionAborted,
			ErrorKind::ConnectionReset => NetworkPeerError::ConnectionReset,
			_ => NetworkPeerError::Unknown
		}
	}
}