#![allow(unused)]
#![no_std]

#[cfg(test)]
fn main() {}

pub mod gdt;
pub mod idt;
pub mod vga;

use core::{panic::PanicInfo, ptr};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn kernel_main() {
    let mut vga_buffer = vga::Buffer::new();
    vga_buffer.clear_screen();
    vga_buffer.set_colors(vga::VGAColor::Black, vga::VGAColor::Green);
    vga_buffer.set_foreground_color(vga::VGAColor::Blue);
    vga_buffer.write_char_at(1, 0, b'4');
    vga_buffer.set_background_color(vga::VGAColor::Red);
    vga_buffer.write_char_at(2, 0, b'7');
    loop {}
}
