#![allow(unused)]
#![no_std]
#[no_mangle]
static GDT_LIMIT: usize = 3;
#[no_mangle]
static mut GDT: [u64; GDT_LIMIT] = [0x0, 0x00CF9A000000FFFF, 0x00CF92000000FFFF];
mod gdt;
mod idt;
mod print;
mod terminal;

use core::{panic::PanicInfo, ptr};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn kernel_main() {
    let mut t = terminal::Vga::new();
    t.clear_screen();
    t.set_background_color(terminal::Color::LightMagenta);
    t.write_u8_arr("du hund".as_bytes());

    loop {}
}

#[cfg(test)]
fn main() {}
