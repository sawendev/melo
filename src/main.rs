// Melo Fantasy Console Emulator

fn main() {
	let start = [               // @[start:$0];
		0xC9, 0x26, 0x00, 0xFF, //   mov16 sp imm $FF00;
		0x88, 0xD6, 0x05,       //   mov r13 imm #5;
		0xCA, 0x06, 0x00, 0x20, //   call pc addr [factorial];
		0x5E, 0x40,             //   set halt;
	];
	let mul = [                 // @[mul];
		0x88, 0xF6, 0x00,       //   mov r15 imm #0;
		                        // @[mul.loop];
		0x41, 0xEE,             //   cmp r14;
		0x42, 0x02,             //   any zero;
		0x6B, 0x00,             //   ret? pc;
		0x5D, 0xE1,             //   dec r14 1;
		0x5F, 0x01,             //   clear carry;
		0x54, 0xFD,             //   add r15 r13;
		0xC9, 0x06, 0x03, 0x10, //   mov16 pc addr [mul.loop];
	];
	let fact = [                // @[factorial];
		0x48, 0xFD,             //   mov r15 r13;
		                        // @[factorial.loop];
		0x81, 0xD6, 0x01,       //   cmp r13 imm #1;
		0x42, 0x18,             //   any (lt | eq);
		0x6B, 0x00,             //   ret? pc;
		0x5D, 0xD1,             //   dec r13 1;
		0x48, 0xEF,             //   mov r14 r15;
		0xCA, 0x06, 0x00, 0x10, //   call pc addr [mul];
		0xC9, 0x06, 0x02, 0x20, //   mov16 pc addr [factorial.loop];
	];
	
	let mut ram = vec![0; 65536];
	
	ram[0x0000..0x0000+start.len()].copy_from_slice(&start);
	ram[0x1000..0x1000+mul.len()].copy_from_slice(&mul);
	ram[0x2000..0x2000+fact.len()].copy_from_slice(&fact);
	
	let mut cpu = melo::mfcpu::MeloCpu::rand();
	
	let start = std::time::Instant::now();
	let mut cc = 0;
	while !cpu.is_halted() {
		//println!("{cpu}");
		cpu.tick(&mut ram);
		cc += 1;
	}
	let dur = start.elapsed();
	
	println!("Executed {cc} instructions in {dur:?}.");
	println!("{cpu}");
}
