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

    fn fetch(&mut self, mem: &ram::Ram) -> u16 {
        // grab instruction
        let inst: u16 = mem.read_mem(self.pc as usize);

        // increase program counter
        self.pc += 2;

        inst
    }

    fn decode(&mut self, inst: u16) -> Instruction {
        let a: u8 = ((inst & 0xf000) >> 12) as u8;
        let b: u8 = ((inst & 0x0f00) >> 8) as u8;
        let c: u8 = ((inst & 0x00f0) >> 4) as u8;
        let d: u8 = (inst & 0x000f) as u8;

        match [a, b, c, d] {
            [0x0, 0x0, 0xE, 0x0] => Instruction::CLS,
            [0x0, 0x0, 0xE, 0xE] => Instruction::RET,
            [0x0, _, _, _] => Instruction::SYS(combine_nibbles(0, b, c, d)),
            [0x1, _, _, _] => Instruction::JP(combine_nibbles(0, b, c, d)),
            [_, _, _, _] => panic!("Can't decode instruction.")
        }
    }
}

fn combine_nibbles(a: u8, b: u8, c: u8, d: u8) -> u16 {
    (a as u16) << 12 | (b as u16) << 8 | (c as u16) << 4 | (d as u16)
}

enum Reg {
    V(u8),
}

enum Instruction {
    CLS,                     //            00E0 - CLS
    RET,                     //            00EE - RET
    SYS(u16),                //            0nnn - SYS addr
    JP(u16),                 //            1nnn - JP addr
    CALL(u16),               //            2nnn - CALL addr
    SE_R_B(Reg, u8),         //            3xkk - SE Vx, byte
    SNE_R_B(Reg, u8),        //            4xkk - SNE Vx, byte
    SE_R_R(Reg, Reg),        //            5xy0 - SE Vx, Vy
    LD_R_B(Reg, u8),         //            6xkk - LD Vx, byte
    ADD_R_B(Reg, u8),        //            7xkk - ADD Vx, byte
    LD_R_R(Reg, Reg),        //            8xy0 - LD Vx, Vy
    OR_R_R(Reg, Reg),        //            8xy1 - OR Vx, Vy
    AND_R_R(Reg, Reg),       //            8xy2 - AND Vx, Vy
    XOR_R_R(Reg, Reg),       //            8xy3 - XOR Vx, Vy
    ADD_R_R(Reg, Reg),       //            8xy4 - ADD Vx, Vy
    SUB_R_R(Reg, Reg),       //            8xy5 - SUB Vx, Vy
    SHR_R_R(Reg, Reg),       //            8xy6 - SHR Vx {, Vy}
    SUBN_R_R(Reg, Reg),      //            8xy7 - SUBN Vx, Vy
    SHL_R_R(Reg, Reg),       //            8xyE - SHL Vx {, Vy}
    SNE_R_R(Reg, Reg),       //            9xy0 - SNE Vx, Vy
    LD_I(u16),               //            Annn - LD I, addr
    JP_V0(u16),              //            Bnnn - JP V0, addr
    RND_R_B(Reg, u8),        //            Cxkk - RND Vx, byte
    DRW_R_R_N(Reg, Reg, u8), //            Dxyn - DRW Vx, Vy, nibble
    SKP_R(Reg),              //            Ex9E - SKP Vx
    SKNP_R(Reg),             //            ExA1 - SKNP Vx
    LD_R_DT(Reg),            //            Fx07 - LD Vx, DT
    LD_R_K(Reg),             //            Fx0A - LD Vx, K
    LD_DT_R(Reg),            //            Fx15 - LD DT, Vx
    LD_ST_R(Reg),            //            Fx18 - LD ST, Vx
    ADD_I_R(Reg),            //            Fx1E - ADD I, Vx
    LD_F_R(Reg),             //            Fx29 - LD F, Vx
    LD_B_R(Reg),             //            Fx33 - LD B, Vx
    LD_MI_R(Reg),            //            Fx55 - LD [I], Vx
    LD_R_MI(Reg),            //            Fx65 - LD Vx, [I]
}
