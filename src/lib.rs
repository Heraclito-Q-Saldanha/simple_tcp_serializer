mod error;
use error::NetworkPeerError;

use std::{
	io::prelude::*,
	net,
};

pub struct NetworkPeer{
	pub stream: net::TcpStream
}

impl NetworkPeer{
	pub fn new(stream: net::TcpStream) -> NetworkPeer{
		NetworkPeer{stream}
	}
	pub fn read<T: serde::de::DeserializeOwned>(&mut self) -> Result<T, NetworkPeerError>{
		let mut size = [0u8; 2];
		self.stream.read_exact(&mut size)?;
		let mut data = vec![0u8; u16::from_be_bytes(size) as usize];
		self.stream.read_exact(&mut data)?;
		Ok(bincode::deserialize(&data)?)
	}
	pub fn write(&mut self, pack: &impl serde::Serialize) -> Result<(), NetworkPeerError>{
		let data = bincode::serialize(pack)?;
		let data = [Vec::from((data.len() as u16).to_be_bytes()), data].concat();
		Ok(self.stream.write_all(&data)?)
	}
}