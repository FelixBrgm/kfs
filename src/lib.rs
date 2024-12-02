#![no_std]

use core::{ops::Deref, ptr::write_volatile};

#[no_mangle]
static GDT_LIMIT: usize = 3;
#[no_mangle]
static mut GDT: [u64; GDT_LIMIT] = [0x0, 0x00CF9A000000FFFF, 0x00CF92000000FFFF];

mod gdt;
mod idt;
mod print;
mod ps2;
mod terminal;
mod vga;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

const VGA_BUFFER_ADDR: *mut u16 = 0xB8000 as *mut u16;

struct Entry {
    color: u8,
    character: u8,
}

impl Entry {
    pub fn new(character: u8) -> Self {
        Entry { color: 0x07, character}
    }

    pub fn to_u16(&self) -> u16 {
        ((self.color as u16) << 8) | (self.character as u16)
    }
}

#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn kernel_main() {
    // t.write_char('H' as u8);
    unsafe { write_volatile(VGA_BUFFER_ADDR.add(2), Entry::new(b'9').to_u16() )};
    // loop {
    //     if let Some(char) = ps2::read_if_ready() {
    //         if char == ps2::Key::Backspace {
    //             t.delete_char();
    //         } else if char == ps2::Key::Enter {
    //             t.new_line();
    //         } else if char == ps2::Key::ArrowLeft {
    //             t.move_cursor(vga::Direction::Left);
    //         } else if char == ps2::Key::ArrowRight {
    //             t.move_cursor(vga::Direction::Right);
    //         } else {
    //             t.write_char(char as u8);
    //         }
    //     }
    // }
    loop {}
}
