use core::ptr::write_volatile;

use super::vga::{flush_vga, Entry};

const BUFFER_SIZE: usize = 1000;

pub struct Terminal {
    // ad a iter from viewport
    pub buffer: [Option<u16>; BUFFER_SIZE],
    cursor: usize,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            buffer: [None; BUFFER_SIZE],
            cursor: 0,
        }
    }

    pub fn write(&mut self, character: u8) {
        self.buffer[self.cursor] = Some(Entry::new(character).to_u16());
        self.cursor += 1;
    }

    pub fn flush(&self) {
        flush_vga(self);
    }
}
