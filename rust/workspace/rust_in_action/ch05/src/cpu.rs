struct CPU<'a> {
    registers: [u8; 0x10],
    pc: usize,
    memory: &'a [u8],
}

impl CPU<'_> {
    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.pc += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            let kk = (opcode & 0x00FF) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => return,
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                (0x7, _, _, _) => self.registers[x as usize] = kk,
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
        } else {
            self.registers[0x0F] = 0;
        }
    }
}

// cargo test cpu_v2 -p ch05 -- --show-output
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut cpu = CPU {
            registers: [0; 16],
            pc: 0,
            memory: &[0; 0],
        };

        #[rustfmt::skip]
        let memory = [
            0x70, 0x05,
            0x71, 0x0A,
            0x72, 0x0A,
            0x73, 0x0A,

            0x80, 0x14,
            0x80, 0x24,
            0x80, 0x34,

            0x00, 0x00,
        ];

        cpu.memory = &memory;
        cpu.pc = 0;
        cpu.run();

        assert_eq!(35, cpu.registers[0]);
        assert_eq!(0, cpu.registers[15]);

        #[rustfmt::skip]
        let memory = [
            0x71, 0xDD,
            0x80, 0x14,
            0x00, 0x00,
        ];

        cpu.memory = &memory;
        cpu.pc = 0;
        cpu.run();

        assert_eq!(0, cpu.registers[0]);
        assert_eq!(1, cpu.registers[15]);

        #[rustfmt::skip]
        let memory = [
            0x71, 0xDD,
            0x80, 0x14,
            0x00, 0x00,
        ];

        cpu.memory = &memory;
        cpu.pc = 0;
        cpu.run();

        assert_eq!(0xDD, cpu.registers[0]);
        assert_eq!(0, cpu.registers[15]);
    }
}
