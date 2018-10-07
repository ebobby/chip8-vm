pub struct Cpu {
    regs: [u8; 16],
    stack: [u16; 16],
    pc: u16,
    sp: u8,
    i: u16,
    dr: u8,
    sr: u8,
}

pub fn new() -> Cpu {
    Cpu {
        regs: [0; 16],
        stack: [0; 16],
        i: 0,
        pc: 0,
        sp: 0,
        dr: 0,
        sr: 0,
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

        self.pc = 0;
        self.sp = 0;
        self.i = 0;
        self.dr = 0;
        self.sr = 0;
    }
}
