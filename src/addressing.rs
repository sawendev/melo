// Addressing trait

pub trait Addressable {
	fn read_byte(&self, addr: u16) -> u8;
	fn write_byte(&mut self, addr: u16, val: u8);
	
	fn read_le_word(&self, addr: u16) -> u16 {
		u16::from_le_bytes([self.read_byte(addr), self.read_byte(addr.wrapping_add(1))])
	}
	
	fn write_le_word(&mut self, addr: u16, val: u16) {
		let [lo, hi] = val.to_le_bytes();
		self.write_byte(addr, lo);
		self.write_byte(addr.wrapping_add(1), hi);
	}
	
	fn read_be_word(&self, addr: u16) -> u16 {
		u16::from_be_bytes([self.read_byte(addr), self.read_byte(addr.wrapping_add(1))])
	}
	
	fn write_be_word(&mut self, addr: u16, val: u16) {
		let [hi, lo] = val.to_be_bytes();
		self.write_byte(addr, hi);
		self.write_byte(addr.wrapping_add(1), lo);
	}
}

impl<T> Addressable for T
where
	T: AsRef<[u8]> + AsMut<[u8]>,
{
	fn read_byte(&self, addr: u16) -> u8 {
		self.as_ref().get(addr as usize).copied().unwrap_or(0)
	}
	
	fn write_byte(&mut self, addr: u16, val: u8) {
		if let Some(byte) = self.as_mut().get_mut(addr as usize) {
			*byte = val;
		}
	}
}
