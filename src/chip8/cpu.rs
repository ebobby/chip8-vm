use chip8::ram;

pub struct Cpu {
    regs: [u8; 16],
    stack: [u16; 16],
    pc: u16,
    sp: u8,
    i: u16,
    dt: u8,
    st: u8,
}

pub fn new() -> Cpu {
    Cpu {
        regs: [0; 16],
        stack: [0; 16],
        i: 0,
        pc: 0,
        sp: 0,
        dt: 0,
        st: 0,
    }
}

impl Cpu {
    pub fn reset(&mut self) {
        // clear regs
        for elem in self.regs.iter_mut() {
            *elem = 0
        }
        // clear stack
        for elem in self.stack.iter_mut() {
            *elem = 0
        }

        self.pc = ::chip8::PROGRAM_START as u16;
        self.sp = 0;
        self.i = 0;
        self.dt = 0;
        self.st = 0;
    }

    pub fn step(&mut self, mem: &ram::Ram) {
        let _inst = self.fetch(mem);
    }

    // private

    fn fetch(&mut self, mem: &ram::Ram) -> [u8; 4] {
        // grab instruction
        let inst: u16 = mem.read_mem(self.pc as usize);

        // increase program counter
        self.pc += 2;

        let a : u8 = ((inst & 0xf000) >> 12) as u8;
        let b : u8 = ((inst & 0x0f00) >> 8) as u8;
        let c : u8 = ((inst & 0x00f0) >> 4) as u8;
        let d : u8 = (inst & 0x000f) as u8;

        [a, b, c, d]
    }
}

//            00E0 - CLS
//            00EE - RET
//            0nnn - SYS addr
//            1nnn - JP addr
//            2nnn - CALL addr
//            3xkk - SE Vx, byte
//            4xkk - SNE Vx, byte
//            5xy0 - SE Vx, Vy
//            6xkk - LD Vx, byte
//            7xkk - ADD Vx, byte
//            8xy0 - LD Vx, Vy
//            8xy1 - OR Vx, Vy
//            8xy2 - AND Vx, Vy
//            8xy3 - XOR Vx, Vy
//            8xy4 - ADD Vx, Vy
//            8xy5 - SUB Vx, Vy
//            8xy6 - SHR Vx {, Vy}
//            8xy7 - SUBN Vx, Vy
//            8xyE - SHL Vx {, Vy}
//            9xy0 - SNE Vx, Vy
//            Annn - LD I, addr
//            Bnnn - JP V0, addr
//            Cxkk - RND Vx, byte
//            Dxyn - DRW Vx, Vy, nibble
//            Ex9E - SKP Vx
//            ExA1 - SKNP Vx
//            Fx07 - LD Vx, DT
//            Fx0A - LD Vx, K
//            Fx15 - LD DT, Vx
//            Fx18 - LD ST, Vx
//            Fx1E - ADD I, Vx
//            Fx29 - LD F, Vx
//            Fx33 - LD B, Vx
//            Fx55 - LD [I], Vx
//
