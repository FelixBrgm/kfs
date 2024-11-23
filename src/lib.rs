#![no_std]

#[no_mangle]
static GDT_LIMIT: usize = 3;
#[no_mangle]
static mut GDT: [u64; GDT_LIMIT] = [0x0, 0x00CF9A000000FFFF, 0x00CF92000000FFFF];

mod gdt;
mod idt;
mod print;
mod ps2;
mod vga;

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn kernel_main() {
    let mut t = vga::Vga::new();
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
mod u64_to_base_test {
    use super::*;

    #[test]
    fn test_normal_functionality_base_16_ff() {
        let num = 255u64;

        let res = match u64_to_base(num, 16) {
            Ok((len, buf)) => (len, buf),
            _ => (0, [0u8; 65]),
        };

        let result_slice = &res.1[65 - res.0..];

        let result_str = core::str::from_utf8(result_slice).unwrap();

        assert_eq!(result_str, "FF");
    }

    #[test]
    fn test_normal_functionality_base_16_ffff() {
        let num = 65535u64;

        let res = match u64_to_base(num, 16) {
            Ok((len, buf)) => (len, buf),
            _ => (0, [0u8; 65]),
        };

        let result_slice = &res.1[65 - res.0..];

        let result_str = core::str::from_utf8(result_slice).unwrap();

        assert_eq!(result_str, "FFFF");
    }

    #[test]
    fn test_normal_functionality_base_16_ffffff() {
        let num = 16777215u64;

        let res = match u64_to_base(num, 16) {
            Ok((len, buf)) => (len, buf),
            _ => (0, [0u8; 65]),
        };

        let result_slice = &res.1[65 - res.0..];

        let result_str = core::str::from_utf8(result_slice).unwrap();

        assert_eq!(result_str, "FFFFFF");
    }

    #[test]
    fn test_normal_functionality_base_16_ffffffff() {
        let num = 4294967295u64;

        let res = match u64_to_base(num, 16) {
            Ok((len, buf)) => (len, buf),
            _ => (0, [0u8; 65]),
        };

        let result_slice = &res.1[65 - res.0..];

        let result_str = core::str::from_utf8(result_slice).unwrap();

        assert_eq!(result_str, "FFFFFFFF");
    }
}
