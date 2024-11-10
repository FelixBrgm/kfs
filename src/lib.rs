#![allow(unused)]
#![no_std]

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
    gdt::init();
    idt::init();

    loop {}
}

#[cfg(test)]
fn main() {}
