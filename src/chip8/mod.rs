mod cpu;
mod ram;

const MEM_SIZE: usize = 4096;
const PROGRAM_START: usize = 0x200;

pub struct Chip8 {
    ram: ram::Ram,
    cpu: cpu::Cpu,
}

pub fn new() -> Chip8 {
    Chip8 {
        ram: ram::new(),
        cpu: cpu::new(),
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
