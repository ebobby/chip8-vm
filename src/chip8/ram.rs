use std::fs::File;
use std::io::prelude::*;

pub struct Ram {
    mem: [u8; ::chip8::MEM_SIZE]
}

pub fn new() -> Ram {
    Ram {
        mem: [0; ::chip8::MEM_SIZE]
    }
}

impl Ram {
    pub fn reset(&mut self) {
        for elem in self.mem.iter_mut() { *elem = 0 }
    }

    pub fn dump_mem(&self) {
        for i in (0..::chip8::MEM_SIZE).step_by(2) {
            println!("0x{:04x}: 0x{:02x}{:02x}", i, self.mem[i], self.mem[i + 1])
        }
    }

    pub fn load_rom(&mut self, path: String) -> ::std::io::Result<()> {
        let mut f = File::open(path)?;

        f.read(&mut self.mem[::chip8::PROGRAM_START..]).expect("Oh noes!");

        Ok(())
    }
}
