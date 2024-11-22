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

use core::{arch::asm, panic::PanicInfo, ptr};

use print::u64_to_base;
use terminal::VGA_WIDTH;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const PS2_DATA_PORT: u16 = 0x60;
const PS2_STATUS_PORT: u16 = 0x64;
const PS2_OUTPUT_BUFFER_STATUS_BIT: u8 = 1;

unsafe fn status(port: u16) -> u8 {
    let res: u8;

    asm!(
        "in al, dx",
        in("dx") port,
        out("al") res,
    );

    res
}

fn buffer_full() -> bool {
    (unsafe { status(PS2_STATUS_PORT) } & PS2_OUTPUT_BUFFER_STATUS_BIT != 0)
}

fn read_if_ready() -> Option<u8> {
    if buffer_full() {
        Some(ps2_read())
    } else {
        None
    }
}

unsafe fn read(port: u16) -> u8 {
    let res: u8;

    asm!(
        "in al, dx",
        in("dx") port,
        out("al") res,
    );

    res
}

fn ps2_read() -> u8 {
    unsafe { read(PS2_DATA_PORT) }
}

const SCANCODE_TO_ASCII: [Option<char>; 58] = [
    None,
    None,
    Some('1'),
    Some('2'),
    Some('3'),
    Some('4'),
    Some('5'),
    Some('6'),
    Some('7'),
    Some('8'),
    Some('9'),
    Some('0'),
    Some('-'),
    Some('='),
    Some(14 as char),
    Some('\t'),
    Some('q'),
    Some('w'),
    Some('e'),
    Some('r'),
    Some('t'),
    Some('y'),
    Some('u'),
    Some('i'),
    Some('o'),
    Some('p'),
    Some('['),
    Some(']'),
    Some(28 as char),
    None,
    Some('a'),
    Some('s'),
    Some('d'),
    Some('f'),
    Some('g'),
    Some('h'),
    Some('j'),
    Some('k'),
    Some('l'),
    Some(';'),
    Some('\''),
    Some('`'),
    None,
    Some('\\'),
    Some('z'),
    Some('x'),
    Some('c'),
    Some('v'),
    Some('b'),
    Some('n'),
    Some('m'),
    Some(','),
    Some('.'),
    Some('/'),
    None,
    Some('*'),
    None,
    Some(' '),
];

#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn kernel_main() {
    let mut t = terminal::Vga::new();
    t.clear_screen();

    let mut is_break_code = false;

    loop {
        if let Some(code) = read_if_ready() {
            if let Some(char) = SCANCODE_TO_ASCII.get(code as usize).and_then(|&opt| opt) {
                if char == 14 as char {
                    t.delete_char();
                } else if char == 28 as char {
                    t.new_line();
                } else {
                    t.write_char(char as u8);
                }
            }
            // else {
            //     let conv = u64_to_base(code as u64, 10).unwrap();
            //     let buf = conv.1;
            //     let len = conv.0;
            //     let num_slice = &buf[buf.len() - len..];
            //     t.write_u8_arr(num_slice);
            // }
        }
    }
}

#[cfg(test)]
fn main() {}
