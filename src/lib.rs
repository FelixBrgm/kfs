#![allow(unused)]
#![no_std]
#[cfg(test)]
fn main() {}

pub mod gdt;
pub mod idt;
pub mod terminal;

#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn kernel_main() {
    let a = kernel_init();

    let mut terminal = Terminal::new();

    terminal.clear_screen();
    terminal.write_u8_arr("Hello World".as_bytes());
    loop {}
}

use core::{panic::PanicInfo, ptr};

use terminal::Terminal;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn kernel_init() -> bool {
    unsafe { gdt::init()}
}
