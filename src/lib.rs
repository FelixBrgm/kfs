#![allow(unused)]
#![no_std]
#[cfg(test)]
fn main() {}

pub mod gdt;
pub mod idt;
pub mod vga;

#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn kernel_main() {
    let a = kernel_init();

    let mut vga_buffer = vga::Buffer::new();
    vga_buffer.clear_screen();
    vga_buffer.set_colors(vga::VGAColor::Black, vga::VGAColor::Green);
    match a {
        true => vga_buffer.set_background_color(vga::VGAColor::Green),
        false => vga_buffer.set_background_color(vga::VGAColor::Red)
    }
    vga_buffer.write_char_at(1, 0, b'4');
    loop {}
}

use core::{panic::PanicInfo, ptr};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn kernel_init() -> bool {
    unsafe { gdt::init()}
}
