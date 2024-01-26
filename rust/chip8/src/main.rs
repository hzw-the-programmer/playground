fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        position_in_memory: 0,
        memory: [0; 0x1000],
    };

    let memory = &mut cpu.memory;

    memory[0] = 0x80;
    memory[1] = 0x14;
    memory[2] = 0x80;
    memory[3] = 0x24;
    memory[4] = 0x80;
    memory[5] = 0x34;

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    cpu.run();

    assert_eq!(cpu.registers[0], 35);

    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}

struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000], // 4096, 4k, 2_i32.pow(12)
}

impl CPU {
    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            match (c, x, y, d) {
                (0x00, 0x00, 0x00, 0x00) => break,
                (0x08, _, _, 0x04) => self.add_xy(x, y),
                _ => todo!("opcode 0x{:04x}", opcode),
            }
        }
    }

    fn read_opcode(&self) -> u16 {
        let pos = self.position_in_memory;
        let b1 = self.memory[pos] as u16;
        let b2 = self.memory[pos + 1] as u16;
        (b1 << 8) | b2
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}
