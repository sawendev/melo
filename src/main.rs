// Melo Fantasy Console Emulator

fn main() {
	let mut ram = [
		0x88, 0x86, 0x20, // mov(r8, $20)
		0x88, 0x96, 0x69, // mov(r9, $69)
		0x48, 0xA8, // mov(r10, r8)
		0x5F, 0x01, // clear(carry)
		0x54, 0xA9, // add(r10, r9)
		0x5E, 0x40, // set(halt)
	];
	
	let mut cpu = melo::mfcpu::MeloCpu::rand();
	
	while !cpu.is_halted() {
		println!("{cpu}");
		cpu.tick(&mut ram);
		std::io::stdin().read_line(&mut String::new()).unwrap();
	}
	
	println!("{cpu}");
}
