use core::ptr::write_volatile;

use super::terminal::Terminal;

const VGA_BUFFER_ADDR: *mut u16 = 0xB8000 as *mut u16;

pub fn flush_vga(t: &Terminal) {
    for (i, &e) in t.buffer.iter().enumerate() {
        if let Some(e) = e {
            unsafe { write_volatile(VGA_BUFFER_ADDR.add(i), e) };
        }
    }
}

pub struct Entry {
    color: u8,
    character: u8,
}

impl Entry {
    pub fn new(character: u8) -> Self {
        Entry {
            color: Color::Default as u8,
            character,
        }
    }

    pub fn to_u16(&self) -> u16 {
        ((self.color as u16) << 8) | (self.character as u16)
    }
}

#[repr(u8)]
enum Color {
    Default = 0x07,
}
