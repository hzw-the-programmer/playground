fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        position_in_memory: 0,
        memory: [0; 0x1000],
        stack: [0; 16],
        stack_pointer: 0,
    };

    let memory = &mut cpu.memory;

    memory[0] = 0x21;
    memory[1] = 0x00;
    memory[2] = 0x21;
    memory[3] = 0x00;
    memory[4] = 0x00;
    memory[5] = 0x00;

    memory[0x100] = 0x80;
    memory[0x101] = 0x14;
    memory[0x102] = 0x80;
    memory[0x103] = 0x14;
    memory[0x104] = 0x00;
    memory[0x105] = 0xEE;

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);

    println!("5 + 10 + 10 + 10 + 10 = {}", cpu.registers[0]);
}

struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000], // 4096, 4k, 2_i32.pow(12)
    stack: [u16; 16],
    stack_pointer: usize,
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

            let nnn = opcode & 0x0FFF;

            match (c, x, y, d) {
                (0x08, _, _, 0x04) => self.add_xy(x, y),
                (0x02, _, _, _) => self.call(nnn),
                (0x00, 0x00, 0x0E, 0x0E) => self.ret(),
                (0x00, 0x00, 0x00, 0x00) => return,
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
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;
        if overflow {
            self.registers[0x0F] = 1;
        } else {
            self.registers[0x0F] = 0;
        }
    }

    fn call(&mut self, addr: u16) {
        if self.stack_pointer >= self.stack.len() {
            panic!("stack overflow");
        }
        self.stack[self.stack_pointer] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("stack underflow");
        }
        self.stack_pointer -= 1;
        self.position_in_memory = self.stack[self.stack_pointer] as usize;
    }
}
