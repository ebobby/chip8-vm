use chip8::ram;
use chip8::utils;

#[derive(Debug)]
pub struct Cpu {
    regs: [u8; 16],
    stack: [u16; 16],
    pc: u16,
    sp: u8,
    i: u16,
}

#[derive(Debug)]
enum Reg {
    V(u8),
}

#[derive(Debug)]
enum Opcode {
    Clear,                       //            00E0 - CLS
    Return,                      //            00EE - RET
    SysCall(u16),                //            0nnn - SYS addr
    Jump(u16),                   //            1nnn - JP addr
    Call(u16),                   //            2nnn - CALL addr
    SkipEqualByte(Reg, u8),      //            3xkk - SE Vx, byte
    SkipNotEqualByte(Reg, u8),   //            4xkk - SNE Vx, byte
    SkipEqual(Reg, Reg),         //            5xy0 - SE Vx, Vy
    LoadByte(Reg, u8),           //            6xkk - LD Vx, byte
    AddByte(Reg, u8),            //            7xkk - ADD Vx, byte
    Copy(Reg, Reg),              //            8xy0 - LD Vx, Vy
    Or(Reg, Reg),                //            8xy1 - OR Vx, Vy
    And(Reg, Reg),               //            8xy2 - AND Vx, Vy
    Xor(Reg, Reg),               //            8xy3 - XOR Vx, Vy
    Add(Reg, Reg),               //            8xy4 - ADD Vx, Vy
    Sub(Reg, Reg),               //            8xy5 - SUB Vx, Vy
    Shr(Reg, Reg),               //            8xy6 - SHR Vx {, Vy}
    Subn(Reg, Reg),              //            8xy7 - SUBN Vx, Vy
    Shl(Reg, Reg),               //            8xyE - SHL Vx {, Vy}
    SkipNotEqual(Reg, Reg),      //            9xy0 - SNE Vx, Vy
    WriteI(u16),                 //            Annn - LD I, addr
    JumpV0(u16),                 //            Bnnn - JP V0, addr
    Random(Reg, u8),             //            Cxkk - RND Vx, byte
    Draw(Reg, Reg, u8),          //            Dxyn - DRW Vx, Vy, nibble
    SkipPressed(Reg),            //            Ex9E - SKP Vx
    SkipNotPressed(Reg),         //            ExA1 - SKNP Vx
    ReadDt(Reg),                 //            Fx07 - LD Vx, DT
    ReadKey(Reg),                //            Fx0A - LD Vx, K
    WriteDt(Reg),                //            Fx15 - LD DT, Vx
    WriteSt(Reg),                //            Fx18 - LD ST, Vx
    AddI(Reg),                   //            Fx1E - ADD I, Vx
    LoadDigit(Reg),              //            Fx29 - LD F, Vx
    WriteBCD(Reg),               //            Fx33 - LD B, Vx
    WriteRegs(Reg),              //            Fx55 - LD [I], Vx
    ReadRegs(Reg),               //            Fx65 - LD Vx, [I]
    NoOp,                        //            *
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
        let mut ops = Vec::new();

        for _i in 0..1024 {
            let prev_pc = self.pc;
            let fetched = self.fetch(mem);
            let decoded = self.decode(fetched);

            ops.push((prev_pc, decoded));
        }

        println!("Decoded instructions: ");
        for op in ops.iter() {
            println!("{:04}: {:?}", op.0, op.1);
        }
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
        match utils::u16_to_nibbles(inst) {
            (0x0, 0x0, 0xE, 0x0) => Opcode::Clear,
            (0x0, 0x0, 0xE, 0xE) => Opcode::Return,
            (0x0, n1, n2, n3) => Opcode::SysCall(utils::nibbles_to_u16(0, n1, n2, n3)),
            (0x1, n1, n2, n3) => Opcode::Jump(utils::nibbles_to_u16(0, n1, n2, n3)),
            (0x2, n1, n2, n3) => Opcode::Call(utils::nibbles_to_u16(0, n1, n2, n3)),
            (0x3, x, n1, n2) => Opcode::SkipEqualByte(Reg::V(x), utils::nibbles_to_u8(n1, n2)),
            (0x4, x, n1, n2) => Opcode::SkipNotEqualByte(Reg::V(x), utils::nibbles_to_u8(n1, n2)),
            (0x5, x, y, 0x0) => Opcode::SkipEqual(Reg::V(x), Reg::V(y)),
            (0x6, x, n1, n2) => Opcode::LoadByte(Reg::V(x), utils::nibbles_to_u8(n1, n2)),
            (0x7, x, n1, n2) => Opcode::AddByte(Reg::V(x), utils::nibbles_to_u8(n1, n2)),
            (0x8, x, y, 0x0) => Opcode::Copy(Reg::V(x), Reg::V(y)),
            (0x8, x, y, 0x1) => Opcode::Or(Reg::V(x), Reg::V(y)),
            (0x8, x, y, 0x2) => Opcode::And(Reg::V(x), Reg::V(y)),
            (0x8, x, y, 0x3) => Opcode::Xor(Reg::V(x), Reg::V(y)),
            (0x8, x, y, 0x4) => Opcode::Add(Reg::V(x), Reg::V(y)),
            (0x8, x, y, 0x5) => Opcode::Sub(Reg::V(x), Reg::V(y)),
            (0x8, x, y, 0x6) => Opcode::Shr(Reg::V(x), Reg::V(y)),
            (0x8, x, y, 0x7) => Opcode::Subn(Reg::V(x), Reg::V(y)),
            (0x8, x, y, 0xE) => Opcode::Shl(Reg::V(x), Reg::V(y)),
            (0x9, x, y, 0x0) => Opcode::SkipNotEqual(Reg::V(x), Reg::V(y)),
            (0xA, n1, n2, n3) => Opcode::WriteI(utils::nibbles_to_u16(0, n1, n2, n3)),
            (0xB, n1, n2, n3) => Opcode::JumpV0(utils::nibbles_to_u16(0, n1, n2, n3)),
            (0xC, x, n1, n2) => Opcode::Random(Reg::V(x), utils::nibbles_to_u8(n1, n2)),
            (0xD, x, y, n1) => Opcode::Draw(Reg::V(x), Reg::V(y), n1),
            (0xE, x, 0x9, 0xE) => Opcode::SkipPressed(Reg::V(x)),
            (0xE, x, 0xA, 0x1) => Opcode::SkipNotPressed(Reg::V(x)),
            (0xF, x, 0x0, 0x7) => Opcode::ReadDt(Reg::V(x)),
            (0xF, x, 0x0, 0xA) => Opcode::ReadKey(Reg::V(x)),
            (0xF, x, 0x1, 0x5) => Opcode::WriteDt(Reg::V(x)),
            (0xF, x, 0x1, 0x8) => Opcode::WriteSt(Reg::V(x)),
            (0xF, x, 0x1, 0xE) => Opcode::AddI(Reg::V(x)),
            (0xF, x, 0x2, 0x9) => Opcode::LoadDigit(Reg::V(x)),
            (0xF, x, 0x3, 0x3) => Opcode::WriteBCD(Reg::V(x)),
            (0xF, x, 0x5, 0x5) => Opcode::WriteRegs(Reg::V(x)),
            (0xF, x, 0x6, 0x5) => Opcode::ReadRegs(Reg::V(x)),
            (_, _, _, _) => Opcode::NoOp,
        }
    }
}
