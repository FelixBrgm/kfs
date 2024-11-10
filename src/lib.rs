#![allow(unused)]
#![no_std]

mod gdt;
mod idt;
mod print;
mod vga;
mod panic;

pub use vga::*;

#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn kernel_main() {
    let mut t = Vga::new();
    t.clear_screen();
    t.set_background_color(vga::Color::LightMagenta);
    t.write_u8_arr("du hund".as_bytes());

    loop {}
}

#[cfg(test)]
fn main() {}