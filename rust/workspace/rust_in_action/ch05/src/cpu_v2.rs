struct CPU {
    registers: [u8; 0x10],
    pc: usize,
    memory: [u8; 0x1000],
}

impl CPU {
    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.pc += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => return,
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn read_opcode(&self) -> u16 {
        let op1 = self.memory[self.pc] as u16;
        let op2 = self.memory[self.pc + 1] as u16;
        op1 << 8 | op2
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let (res, overflow) =
            self.registers[x as usize].overflowing_add(self.registers[y as usize]);
        self.registers[x as usize] = res;
        if overflow {
            self.registers[0x0F] = 1;
        }
    }
}

// cargo test cpu_v2 -p ch05 -- --show-output
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut cpu = CPU {
            registers: [0; 16],
            pc: 0,
            memory: [0; 4096],
        };

        cpu.registers[0] = 5;
        cpu.registers[1] = 10;
        cpu.registers[2] = 10;
        cpu.registers[3] = 10;

        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x14;
        cpu.memory[2] = 0x80;
        cpu.memory[3] = 0x24;
        cpu.memory[4] = 0x80;
        cpu.memory[5] = 0x34;

        cpu.run();

        assert_eq!(35, cpu.registers[0]);
        assert_eq!(0, cpu.registers[15]);
    }

    #[test]
    fn test_2() {
        let mut cpu = CPU {
            registers: [0; 16],
            pc: 0,
            memory: [0; 4096],
        };

        cpu.registers[0] = 5;
        cpu.registers[1] = 10;
        cpu.registers[2] = 10;
        cpu.registers[3] = 231;
        // cpu.registers[3] = 232;

        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x14;
        cpu.memory[2] = 0x80;
        cpu.memory[3] = 0x24;
        cpu.memory[4] = 0x80;
        cpu.memory[5] = 0x34;

        cpu.run();

        assert_eq!(0, cpu.registers[0]);
        // assert_eq!(1, cpu.registers[0]);
        assert_eq!(1, cpu.registers[15]);
    }
}
