// RAM and ROM structs

use crate::addressing::*;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct Ram(Vec<u8>);

impl Ram {
	pub fn new(data: Vec<u8>) -> Self {
		Self(data)
	}
	
	pub fn zero(size: usize) -> Self {
		Self(vec![0; size])
	}
	
	pub fn rand(size: usize) -> Self {
		Self(vec![rand::random(); size])
	}
}

impl Addressable for Ram {
	fn read_byte(&self, addr: u16) -> u8 {
		self.0.read_byte(addr)
	}
	
	fn write_byte(&mut self, addr: u16, val: u8) {
		self.0.write_byte(addr, val);
	}
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct Rom(Vec<u8>);

impl Rom {
	pub fn new(data: Vec<u8>) -> Self {
		Self(data)
	}
}

impl Addressable for Rom {
	fn read_byte(&self, addr: u16) -> u8 {
		self.0.read_byte(addr)
	}
	
	fn write_byte(&mut self, _: u16, _: u8) {}
}
