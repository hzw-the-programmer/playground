struct CPU<'a> {
    registers: [u8; 0x10],
    memory: &'a mut [u8],
    pc: usize,
    sp: usize,
}

impl<'a> CPU<'a> {
    fn new(memory: &'a mut [u8], pc: usize, sp: usize) -> CPU<'a> {
        Self {
            registers: [0; 0x10],
            pc,
            sp,
            memory,
        }
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.pc += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            let nnn = opcode & 0x0FFF;
            let kk = (opcode & 0x00FF) as u8;

            match (c, x, y, d) {
                (0x7, _, _, _) => self.set(x, kk),
                (0x8, _, _, 0x4) => self.add(x, y),
                (0x2, _, _, _) => self.call(nnn),
                (0, 0, 0xE, 0xE) => self.ret(),
                (0, 0, 0, 0) => return,
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn read_opcode(&self) -> u16 {
        let op1 = self.memory[self.pc] as u16;
        let op2 = self.memory[self.pc + 1] as u16;
        op1 << 8 | op2
    }

    fn add(&mut self, x: u8, y: u8) {
        let (res, overflow) =
            self.registers[x as usize].overflowing_add(self.registers[y as usize]);
        self.registers[x as usize] = res;
        if overflow {
            self.registers[0x0F] = 1;
        } else {
            self.registers[0x0F] = 0;
        }
    }

    fn set(&mut self, x: u8, v: u8) {
        self.registers[x as usize] = v;
    }

    fn call(&mut self, addr: u16) {
        // println!("call: {:04x} -> {:04x}", self.pc, addr);
        self.memory[self.sp] = (self.pc >> 8) as u8;
        self.memory[self.sp + 1] = self.pc as u8;
        self.sp += 2;
        self.pc = addr as usize;
    }

    fn ret(&mut self) {
        self.sp -= 2;
        let h = self.memory[self.sp] as usize;
        let l = self.memory[self.sp + 1] as usize;
        self.pc = h << 8 | l;
        // println!("ret: {:04x}", self.pc);
    }
}

// cargo test cpu -p ch05 -- --show-output
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        #[rustfmt::skip]
        let mut memory = [
            // main
            0x70, 0x05,
            0x20, 0x10, // call fun_1
            0x20, 0x10, // call fun_1
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00,

            // fun_1: 0x10
            0x71, 0x0A,
            0x72, 0x0A,
            0x73, 0x0A,
            0x20, 0x20, // call fun_2
            0x00, 0xEE,
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00,

            // fun_2: 0x20
            0x80, 0x14,
            0x80, 0x24,
            0x80, 0x34,
            0x00, 0xEE,
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00,

            // stack: 0x2F
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00,
            0x00, 0x00,
        ];

        let mut cpu = CPU::new(&mut memory, 0, 0x2F);
        cpu.run();
        assert_eq!(65, cpu.registers[0]);
        assert_eq!(0, cpu.registers[0x0F]);
    }
}
