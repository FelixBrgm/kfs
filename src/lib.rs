#![allow(unused)]
#![no_std]

#[cfg(test)]
fn main() {}

pub mod gdt;
pub mod idt;
pub mod terminal;

pub fn u64_to_base(mut addr: u64, base: u8) -> Result<(usize, [u8; 65]), ()> {
    if base < 2 || base > 16 {
        return Err(());
    }

    let mut buf: [u8; 65] = [0; 65];
    let digits: &[u8; 16] = b"0123456789ABCDEF";

    let mut idx = buf.len();

    while addr != 0 && idx > 0 {
        idx -= 1;
        buf[idx] = digits[(addr % base as u64) as usize];
        addr /= base as u64;
    }

    if addr != 0 {
        return Err(());
    }

    let len = buf.len() - idx;

    Ok((len, buf))
}

#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn kernel_main() {
    let a = kernel_init();

    loop {}
}

use core::{panic::PanicInfo, ptr};

use terminal::Terminal;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn kernel_init() {
    unsafe { gdt::init() }
    idt::init();
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
