#![no_std]

#[no_mangle]
static GDT_LIMIT: usize = 3;
#[no_mangle]
static mut GDT: [u64; GDT_LIMIT] = [0x0, 0x00CF9A000000FFFF, 0x00CF92000000FFFF];

mod gdt;
mod idt;
mod print;
mod ps2;
mod terminal;

use core::panic::PanicInfo;

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

    loop {
        if let Some(char) = ps2::read_if_ready() {
            if char == ps2::BACKSPACE {
                t.delete_char();
            } else if char == ps2::ENTER {
                t.new_line();
            } else {
                t.write_char(char as u8);
            }
        }
        // Use this block of code to see the scancode for each keypress
        // else {
        //     let conv = u64_to_base(code as u64, 10).unwrap();
        //     let buf = conv.1;
        //     let len = conv.0;
        //     let num_slice = &buf[buf.len() - len..];
        //     t.write_u8_arr(num_slice);
        // }
    }
}

#[cfg(test)]
fn main() {}
