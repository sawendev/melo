// Melo Fantasy CPU

use crate::addressing::Addressable;

pub mod reg_indices {
	pub const PC: u8 = 0;
	pub const SP: u8 = 2;
	pub const FLAG: u8 = 4;
	pub const A0: u8 = 5;
	pub const A1: u8 = 6;
	pub const A2: u8 = 7;
}

pub mod flag_masks {
	pub const COND: u8 = 1 << 7;
	pub const HALT: u8 = 1 << 6;
	pub const GT: u8 = 1 << 5;
	pub const EQ: u8 = 1 << 4;
	pub const LT: u8 = 1 << 3;
	pub const NEG: u8 = 1 << 2;
	pub const ZERO: u8 = 1 << 1;
	pub const CARRY: u8 = 1 << 0;
}

use flag_masks::*;
use reg_indices::*;

fn nibs(b: u8) -> (u8, u8) {
	(b >> 4, b & 0b1111)
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Default, Debug)]
pub struct MeloCpu {
	regs: [u8; 16],
}

impl std::fmt::Display for MeloCpu {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "PC: ${:04x}, SP: ${:04x}", self.get_le_reg(PC), self.get_le_reg(SP))?;
		writeln!(f, "FLAG: %{:08b}", self.get_reg(FLAG))?;
		writeln!(
			f, "A0: ${:02x}, A1: ${:02x}, A2: ${:02x}",
			self.get_reg(A0), self.get_reg(A1), self.get_reg(A2),
		)?;
		writeln!(
			f, "R8: ${:02x}, R9: ${:02x}, R10: ${:02x}, R11: ${:02x}",
			self.regs[8], self.regs[9], self.regs[10], self.regs[11],
		)?;
		write!(
			f, "R12: ${:02x}, R13: ${:02x}, R14: ${:02x}, R15: ${:02x}",
			self.regs[12], self.regs[13], self.regs[14], self.regs[15],
		)
	}
}

impl MeloCpu {
	pub fn new() -> Self { Self::default() }
	pub fn zero() -> Self { Self { regs: [0; 16] } }
	pub fn rand() -> Self {
		let mut melo = Self { regs: rand::random() };
		melo.reset();
		melo
	}
	
	pub fn reset(&mut self) {
		self.set_le_reg(PC, 0);
		self.clear_flag(HALT);
	}
	
	pub fn halt(&mut self) {
		self.set_flag(HALT);
	}
	
	pub fn clear_halt(&mut self) {
		self.clear_flag(HALT);
	}
	
	pub fn is_halted(&mut self) -> bool {
		self.get_flag(HALT) != 0
	}
	
	fn get_reg(&self, idx: u8) -> u8 { self.regs.read_byte(idx as u16) }
	fn set_reg(&mut self, idx: u8, val: u8) { self.regs.write_byte(idx as u16, val) }
	fn inc_reg(&mut self, idx: u8, amt: u8) { self.set_reg(idx, self.get_reg(idx).wrapping_add(amt)) }
	fn dec_reg(&mut self, idx: u8, amt: u8) { self.set_reg(idx, self.get_reg(idx).wrapping_sub(amt)) }
	
	fn get_le_reg(&self, idx: u8) -> u16 { self.regs.read_le_word(idx as u16 & !1) }
	fn set_le_reg(&mut self, idx: u8, val: u16) { self.regs.write_le_word(idx as u16 & !1, val) }
	fn inc_le_reg(&mut self, idx: u8, amt: u16) { self.set_le_reg(idx, self.get_le_reg(idx).wrapping_add(amt)) }
	
	fn get_flag(&mut self, flag: u8) -> u8 { self.get_reg(FLAG) & flag }
	fn set_flag(&mut self, flag: u8) { self.set_reg(FLAG, self.get_reg(FLAG) | flag) }
	fn clear_flag(&mut self, flag: u8) { self.set_reg(FLAG, self.get_reg(FLAG) & !flag) }
	
	fn fetch(&mut self, bus: &impl Addressable) -> u8 {
		let byte = bus.read_byte(self.get_le_reg(PC));
		self.inc_le_reg(PC, 1);
		byte
	}
	
	fn update_math_flags(&mut self, val: u16) {
		if (val as i8) < 0 { self.set_flag(NEG) } else { self.clear_flag(NEG) }
		if val as u8 == 0 { self.set_flag(ZERO) } else { self.clear_flag(ZERO) }
		if val > 0xFF { self.set_flag(CARRY) } else { self.clear_flag(CARRY) }
	}
	
	fn update_logic_flags(&mut self, val: u8) {
		if (val as i8) < 0 { self.set_flag(NEG) } else { self.clear_flag(NEG) }
		if val == 0 { self.set_flag(ZERO) } else { self.clear_flag(ZERO) }
	}
	
	pub fn tick(&mut self, bus: &mut impl Addressable) {
		if self.get_flag(HALT) != 0 { return }
		
		let opcode = self.fetch(bus);
		let argc = opcode >> 6;
		let cond = (opcode >> 5) & 1;
		
		if argc >= 1 { let b = self.fetch(bus); self.set_reg(A0, b) }
		if argc >= 2 { let b = self.fetch(bus); self.set_reg(A1, b) }
		if argc >= 3 { let b = self.fetch(bus); self.set_reg(A2, b) }
		
		if cond == 0 || self.get_flag(COND) != 0 {
			self.execute(opcode, bus)
		}
	}
	
	fn execute(&mut self, opcode: u8, bus: &mut impl Addressable) {
		let arg = self.get_reg(A0);
		let (dest, src) = nibs(arg);
		match opcode & 0b11111 {
			0x00 => self.nop(),
			0x01 => self.cmp(dest, src),
			0x02 => self.any(arg),
			0x03 => self.all(arg),
			0x04 => self.swap(dest, src),
			0x05 => self.rev(dest, src),
			0x06 => self.zeros(dest, src),
			0x07 => self.ones(dest, src),
			0x08 => self.mov(dest, src),
			0x09 => self.mov16(dest, src),
			0x0A => self.call(dest, src, bus),
			0x0B => self.ret(dest, bus),
			0x0C => self.load(dest, src, bus),
			0x0D => self.store(dest, src, bus),
			0x0E => self.push(src, bus),
			0x0F => self.pop(dest, bus),
			0x10 => self.and(dest, src),
			0x11 => self.or(dest, src),
			0x12 => self.xor(dest, src),
			0x13 => self.not(dest, src),
			0x14 => self.add(dest, src),
			0x15 => self.sub(dest, src),
			0x16 => self.rsub(dest, src),
			0x17 => self.neg(dest, src),
			0x18 => self.shl(dest, src),
			0x19 => self.shr(dest, src),
			0x1A => self.shlimm(dest, src),
			0x1B => self.shrimm(dest, src),
			0x1C => self.inc(dest, src),
			0x1D => self.dec(dest, src),
			0x1E => self.set(arg),
			0x1F => self.clear(arg),
			_ => unreachable!("opcode masked to lower 5 bits"),
		}
	}
	
	fn nop(&self) {}
	
	fn cmp(&mut self, dest: u8, src: u8) {
		let (lhs, rhs) = (self.get_reg(dest), self.get_reg(src));
		if lhs > rhs { self.set_flag(GT) } else { self.clear_flag(GT) }
		if lhs == rhs { self.set_flag(EQ) } else { self.clear_flag(EQ) }
		if lhs < rhs { self.set_flag(LT) } else { self.clear_flag(LT) }
		self.update_logic_flags(rhs);
	}
	
	fn any(&mut self, arg: u8) {
		if self.get_flag(arg) != 0 {
			self.set_flag(COND);
		} else {
			self.clear_flag(COND);
		}
	}
	
	fn all(&mut self, arg: u8) {
		if self.get_flag(arg) == arg {
			self.set_flag(COND);
		} else {
			self.clear_flag(COND);
		}
	}
	
	fn swap(&mut self, dest: u8, src: u8) {
		let (lhs, rhs) = (self.get_reg(dest), self.get_reg(src));
		self.set_reg(dest, rhs);
		self.set_reg(src, lhs);
	}
	
	fn rev(&mut self, dest: u8, src: u8) {
		let val = self.get_reg(src).reverse_bits();
		self.set_reg(dest, val);
	}
	
	fn zeros(&mut self, dest: u8, src: u8) {
		let val = self.get_reg(src).count_zeros() as u8;
		self.set_reg(dest, val);
	}
	
	fn ones(&mut self, dest: u8, src: u8) {
		let val = self.get_reg(src).count_ones() as u8;
		self.set_reg(dest, val);
	}
	
	fn mov(&mut self, dest: u8, src: u8) {
		self.set_reg(dest, self.get_reg(src));
	}
	
	fn mov16(&mut self, dest: u8, src: u8) {
		self.set_le_reg(dest, self.get_le_reg(src));
	}
	
	fn call(&mut self, dest: u8, src: u8, bus: &mut impl Addressable) {
		let ret = self.get_le_reg(dest);
		bus.write_le_word(self.get_le_reg(SP), ret);
		self.inc_reg(SP, 2);
		self.set_le_reg(dest, self.get_le_reg(src));
	}
	
	fn ret(&mut self, dest: u8, bus: &impl Addressable) {
		self.dec_reg(SP, 2);
		let ret = bus.read_le_word(self.get_le_reg(SP));
		self.set_le_reg(dest, ret);
	}
	
	fn load(&mut self, dest: u8, src: u8, bus: &impl Addressable) {
		let byte = bus.read_byte(self.get_le_reg(src));
		self.set_reg(dest, byte);
	}
	
	fn store(&mut self, dest: u8, src: u8, bus: &mut impl Addressable) {
		let byte = self.get_reg(src);
		bus.write_byte(self.get_le_reg(dest), byte);
	}
	
	fn push(&mut self, src: u8, bus: &mut impl Addressable) {
		let byte = self.get_reg(src);
		bus.write_byte(self.get_le_reg(SP), byte);
		self.inc_reg(SP, 1);
	}
	
	fn pop(&mut self, dest: u8, bus: &impl Addressable) {
		self.dec_reg(SP, 1);
		let byte = bus.read_byte(self.get_le_reg(SP));
		self.set_reg(dest, byte);
	}
	
	fn and(&mut self, dest: u8, src: u8) {
		let (lhs, rhs) = (self.get_reg(dest), self.get_reg(src));
		let val = lhs & rhs;
		self.update_logic_flags(val);
		self.set_reg(dest, val);
	}
	
	fn or(&mut self, dest: u8, src: u8) {
		let (lhs, rhs) = (self.get_reg(dest), self.get_reg(src));
		let val = lhs | rhs;
		self.update_logic_flags(val);
		self.set_reg(dest, val);
	}
	
	fn xor(&mut self, dest: u8, src: u8) {
		let (lhs, rhs) = (self.get_reg(dest), self.get_reg(src));
		let val = lhs ^ rhs;
		self.update_logic_flags(val);
		self.set_reg(dest, val);
	}
	
	fn not(&mut self, dest: u8, src: u8) {
		let val = !self.get_reg(src);
		self.update_logic_flags(val);
		self.set_reg(dest, val);
	}
	
	fn add(&mut self, dest: u8, src: u8) {
		let (lhs, rhs) = (self.get_reg(dest), self.get_reg(src));
		let carry = self.get_flag(CARRY);
		let sum = lhs as u16 + rhs as u16 + carry as u16;
		self.update_math_flags(sum);
		self.set_reg(dest, sum as u8);
	}
	
	fn sub(&mut self, dest: u8, src: u8) {
		let (lhs, rhs) = (self.get_reg(dest), !self.get_reg(src));
		let carry = self.get_flag(CARRY);
		let sum = lhs as u16 + rhs as u16 + carry as u16;
		self.update_math_flags(sum);
		self.set_reg(dest, sum as u8);
	}
	
	fn rsub(&mut self, dest: u8, src: u8) {
		let (lhs, rhs) = (!self.get_reg(dest), self.get_reg(src));
		let carry = self.get_flag(CARRY);
		let sum = lhs as u16 + rhs as u16 + carry as u16;
		self.update_math_flags(sum);
		self.set_reg(dest, sum as u8);
	}
	
	fn neg(&mut self, dest: u8, src: u8) {
		let val = 0u8.wrapping_sub(self.get_reg(src));
		self.update_logic_flags(val);
		self.set_reg(dest, val);
	}
	
	fn shl(&mut self, dest: u8, src: u8) {
		let (lhs, rhs) = (self.get_reg(dest), self.get_reg(src));
		let val = lhs.unbounded_shl(rhs as u32);
		self.set_reg(dest, val);
	}
	
	fn shr(&mut self, dest: u8, src: u8) {
		let (lhs, rhs) = (self.get_reg(dest), self.get_reg(src));
		let val = lhs.unbounded_shr(rhs as u32);
		self.set_reg(dest, val);
	}
	
	fn shlimm(&mut self, dest: u8, src: u8) {
		let lhs = self.get_reg(dest);
		let val = lhs.unbounded_shl(src as u32);
		self.set_reg(dest, val);
	}
	
	fn shrimm(&mut self, dest: u8, src: u8) {
		let lhs = self.get_reg(dest);
		let val = lhs.unbounded_shr(src as u32);
		self.set_reg(dest, val);
	}
	
	fn inc(&mut self, dest: u8, src: u8) {
		let (lhs, rhs) = (self.get_reg(dest), src);
		let sum = lhs as u16 + rhs as u16;
		self.update_math_flags(sum);
		self.set_reg(dest, sum as u8);
	}
	
	fn dec(&mut self, dest: u8, src: u8) {
		let (lhs, rhs) = (self.get_reg(dest), !src);
		let sum = lhs as u16 + rhs as u16 + 1;
		self.update_math_flags(sum);
		self.set_reg(dest, sum as u8);
	}
	
	fn set(&mut self, arg: u8) {
		self.set_flag(arg);
	}
	
	fn clear(&mut self, arg: u8) {
		self.clear_flag(arg);
	}
}
