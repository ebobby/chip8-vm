mod cpu;
mod ram;

const MEM_SIZE: usize = 4096;
const PROGRAM_START: usize = 0x200;

pub struct Chip8 {
    cpu: cpu::Cpu,
    ram: ram::Ram,
}

pub fn new() -> Chip8 {
    Chip8 {
        cpu: cpu::new(),
        ram: ram::new(),
    }
}

impl Chip8 {
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.ram.reset();
    }

    pub fn dump_mem(&self) {
        self.ram.dump_mem();
    }

    pub fn load_rom(&mut self, path: String) -> ::std::io::Result<()> {
        self.ram.load_rom(path)
    }
}
