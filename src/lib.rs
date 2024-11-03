#![allow(unused)]
#![no_std]
#![no_main]

pub mod idt;
pub mod vga;

use core::{panic::PanicInfo, ptr};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() {
    let mut vga_buffer = vga::Buffer::new();

    vga_buffer.putstr("42asdhasd");
    loop {}
}

fn main() {}
