use core::arch::asm;

use crate::{print, vga::Vga};

pub const PS2_DATA_PORT: u16 = 0x60;
pub const PS2_STATUS_PORT: u16 = 0x64;
pub const PS2_OUTPUT_BUFFER_STATUS_BIT: u8 = 1;

/// Reads from `PS2_STATUS_PORT` and returns the extracted value.
fn status() -> u8 {
    let res: u8;

    unsafe {
        res = read(PS2_STATUS_PORT);
    }

    res
}

/// Returns `true` if the least significant bit of the ps2 status port is set,
/// meaning it has been written to.
fn buffer_full() -> bool {
    status() & PS2_OUTPUT_BUFFER_STATUS_BIT != 0
}

/// Reads from the PS2 data port if the PS2 status port is ready. Returns `Some(char)`
/// if the converted scancode is a supported character.
pub fn read_if_ready(t: &mut Vga, display_code: bool) -> Option<char> {
    if !buffer_full() {
        return None;
    }

    let code = unsafe { read(PS2_DATA_PORT) };

    // if display_code {
    //     let conv = print::u64_to_base(code as u64, 10).unwrap();
    //     let buf = conv.1;
    //     let len = conv.0;
    //     let num_slice = &buf[buf.len() - len..];
    //     t.write_char(b'|');
    //     t.write_u8_arr(num_slice);
    //     t.write_char(b'|');
    // }

    if let Some(char) = SCANCODE_TO_ASCII.get(code as usize).and_then(|&opt| opt) {
        return Some(char);
    }

    None
}

/// Reads from `port` and returns the extracted value.
/// ## SAFETY:
/// `port` is assumed to be one of `PS2_STATUS_PORT` or `PS2_DATA_PORT`. Passing another value
/// to this function will result in undefines behavior.
unsafe fn read(port: u16) -> u8 {
    let res: u8;

    asm!(
        "in al, dx",
        in("dx") port,
        out("al") res,
    );

    res
}

pub const BACKSPACE: char = 14 as char;
pub const ENTER: char = 28 as char;
pub const ARROW_LEFT: char = 75 as char;
pub const ARROW_UP: char = 72 as char;
pub const ARROW_RIGHT: char = 77 as char;
pub const ARROW_DOWN: char = 80 as char;

/// Conversion table for all characters currently supported by our kernel for PS2 input.
const SCANCODE_TO_ASCII: [Option<char>; 256] = [
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
    Some(BACKSPACE),
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
    Some(ENTER),
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
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(ARROW_UP),
    None,
    None,
    Some(ARROW_LEFT),
    None,
    Some(ARROW_RIGHT),
    None,
    None,
    Some(ARROW_DOWN),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];
