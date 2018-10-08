use chip8::ram;
use chip8::utils;

pub struct Cpu {
    regs: [u8; 16],
    stack: [u16; 16],
    pc: u16,
    sp: u8,
    i: u16,
}

pub fn new() -> Cpu {
    Cpu {
        regs: [0; 16],
        stack: [0; 16],
        i: 0,
        pc: 0,
        sp: 0,
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
    }

    pub fn run_test(&mut self, mem: &ram::Ram) {
        let mut ops= Vec::new();

        for _i in 0..1024 {
            let fetched = self.fetch(mem);
            let decoded = self.decode(fetched);

            ops.push(decoded);
        }

        println!("Decoded instructions: ");
        println!("{:#?}", ops);
    }

    // private

    fn fetch(&mut self, mem: &ram::Ram) -> u16 {
        // grab instruction
        let inst: u16 = mem.read_mem(self.pc as usize);

        // increase program counter
        self.pc += 2;

        inst
    }

    fn decode(&self, inst: u16) -> Opcode {
        let parts = utils::u16_to_nibbles(inst);

        match parts {
            (0x0, 0x0, 0xE, 0x0) => Opcode::Cls,
            (0x0, 0x0, 0xE, 0xE) => Opcode::Ret,
            (0x0, n1, n2, n3) => Opcode::Sys(utils::nibbles_to_u16(0, n1, n2, n3)),
            (0x1, n1, n2, n3) => Opcode::Jp(utils::nibbles_to_u16(0, n1, n2, n3)),
            (0x2, n1, n2, n3) => Opcode::Call(utils::nibbles_to_u16(0, n1, n2, n3)),
            (0x3, x, n1, n2) => Opcode::SeRB(Reg::V(x), utils::nibbles_to_u8(n1, n2)),
            (_, _, _, _) => Opcode::Noop,
        }
    }
}

#[derive(Debug)]
enum Reg {
    V(u8),
}

#[derive(Debug)]
enum Opcode {
    Cls,                  //            00E0 - CLS
    Ret,                  //            00EE - RET
    Sys(u16),             //            0nnn - SYS addr
    Jp(u16),              //            1nnn - JP addr
    Call(u16),            //            2nnn - CALL addr
    SeRB(Reg, u8),        //            3xkk - SE Vx, byte
    SneRB(Reg, u8),       //            4xkk - SNE Vx, byte
    SeRR(Reg, Reg),       //            5xy0 - SE Vx, Vy
    LdRB(Reg, u8),        //            6xkk - LD Vx, byte
    AddRB(Reg, u8),       //            7xkk - ADD Vx, byte
    LdRR(Reg, Reg),       //            8xy0 - LD Vx, Vy
    OrRR(Reg, Reg),       //            8xy1 - OR Vx, Vy
    AndRR(Reg, Reg),      //            8xy2 - AND Vx, Vy
    XorRR(Reg, Reg),      //            8xy3 - XOR Vx, Vy
    AddRR(Reg, Reg),      //            8xy4 - ADD Vx, Vy
    SubRR(Reg, Reg),      //            8xy5 - SUB Vx, Vy
    ShrRR(Reg, Reg),      //            8xy6 - SHR Vx {, Vy}
    SubnRR(Reg, Reg),     //            8xy7 - SUBN Vx, Vy
    ShlRR(Reg, Reg),      //            8xyE - SHL Vx {, Vy}
    SneRR(Reg, Reg),      //            9xy0 - SNE Vx, Vy
    LdI(u16),             //            Annn - LD I, addr
    JpV0(u16),            //            Bnnn - JP V0, addr
    RndRB(Reg, u8),       //            Cxkk - RND Vx, byte
    DrwRRN(Reg, Reg, u8), //            Dxyn - DRW Vx, Vy, nibble
    SkpR(Reg),            //            Ex9E - SKP Vx
    SknpR(Reg),           //            ExA1 - SKNP Vx
    LdRDt(Reg),           //            Fx07 - LD Vx, DT
    LdRK(Reg),            //            Fx0A - LD Vx, K
    LdDtR(Reg),           //            Fx15 - LD DT, Vx
    LdStR(Reg),           //            Fx18 - LD ST, Vx
    AddIR(Reg),           //            Fx1E - ADD I, Vx
    LdFR(Reg),            //            Fx29 - LD F, Vx
    LdBR(Reg),            //            Fx33 - LD B, Vx
    LdIdR(Reg),           //            Fx55 - LD [I], Vx
    LdRId(Reg),           //            Fx65 - LD Vx, [I]
    Noop,
}
