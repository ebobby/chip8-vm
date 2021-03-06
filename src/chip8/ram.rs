use std::fs::File;
use std::io::prelude::*;

const FONT_SPRITES: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Ram {
    mem: [u8; ::chip8::MEM_SIZE],
}

pub fn new() -> Ram {
    Ram {
        mem: [0; ::chip8::MEM_SIZE],
    }
}

impl Ram {
    pub fn reset(&mut self) {
        for elem in self.mem.iter_mut() {
            *elem = 0
        }

        self.init_char_sprites();
    }

    pub fn dump_mem(&self) {
        for i in (0..::chip8::MEM_SIZE).step_by(2) {
            println!("0x{:04x}: 0x{:02x}{:02x}", i, self.mem[i], self.mem[i + 1])
        }
    }

    pub fn load_rom(&mut self, path: String) -> ::std::io::Result<()> {
        let mut f = File::open(path)?;

        f.read(&mut self.mem[::chip8::PROGRAM_START..])
            .expect("Oh noes!");

        Ok(())
    }

    pub fn read_mem(&self, i: usize) -> u16 {
        (self.mem[i] as u16) << 8 | self.mem[i + 1] as u16
    }

    // private

    fn init_char_sprites(&mut self) {
        // insert font sprites into ram
        self.mem[..FONT_SPRITES.len()].clone_from_slice(&FONT_SPRITES);
    }
}
