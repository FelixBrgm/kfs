#![allow(unused)]
#![no_std]
#![no_main]

pub mod vga;

use core::{panic::PanicInfo, ptr};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() {
    let mut vga_buffer = vga::Buffer::new();

    vga_buffer.putchar(b'H');
    vga_buffer.putchar(b'e');
    vga_buffer.putchar(b'l');
    vga_buffer.putchar(b'l');
    vga_buffer.putchar(b'o');
    vga_buffer.putchar(b',');
    vga_buffer.putchar(b' ');
    vga_buffer.putchar(b'W');
    vga_buffer.putchar(b'o');
    vga_buffer.putchar(b'r');
    vga_buffer.putchar(b'l');
    vga_buffer.putchar(b'd');
    vga_buffer.putchar(b'!');
    loop {}
}
