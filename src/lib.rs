#![allow(unused)]
#![no_std]
#![no_main]

use core::{panic::PanicInfo, ptr};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum VGAColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    LightBrown = 14,
    White = 15,
}

const VGA_WIDTH: u16 = 80;
const VGA_HEIGHT: u16 = 25;
const VGA_BUFFER: *mut u16 = 0xB8000 as *mut u16;

struct Terminal {
    color: u8,
    row: usize,
    col: usize,
    buf: *mut u16,
}

unsafe fn strlen(s: *mut u16) -> isize {
    let mut len: isize = 0;

    while s.add(len as usize).read() != 0 {
        len += 1;
    }
    len
}

fn vga_entry_color(foreground: VGAColor, background: VGAColor) -> u8 {
    (foreground as u8) | (background as u8) << 4
}

fn vga_entry(c: u8, color: u8) -> u16 {
    (c as u16) | (color as u16) << 8
}

impl Terminal {
    fn new() -> Self {
        let mut terminal = Terminal {
            color: vga_entry_color(VGAColor::LightGrey, VGAColor::Black),
            row: 0,
            col: 0,
            buf: 0xB8000 as *mut u16,
        };
        terminal.clear_screen();
        terminal
    }

    fn clear_screen(&mut self) {
        for row in 0..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                // Safety:
                // It is still way too early, I am not sure this is safe but I don't know how else to do it
                unsafe {
                    *self.buf.offset((row * VGA_WIDTH + col) as isize) = vga_entry(b' ', self.color)
                }
            }
        }
    }

    fn set_color(&mut self, color: u8) {
        self.color = color;
    }

    fn put_entry_at(&mut self, c: u8, col: u8, row: u8) {
        let index = row * VGA_WIDTH as u8 + col;
        unsafe { *self.buf.offset(index as isize) = vga_entry(c, self.color) }
    }

    fn putchar(&mut self, c: u8) {
        self.put_entry_at(c, self.col as u8, self.row as u8);
        self.col += 1;

        if self.col >= VGA_WIDTH as usize {
            self.col = 0;
            self.row += 1;
            if self.row >= VGA_HEIGHT as usize {
                self.row = 0;
            }
        }
    }

    fn write(&mut self, data: *mut u16, size: isize) {
        for idx in 0..size {
            unsafe {
                self.putchar(*data.offset(idx) as u8);
            }
        }
    }

    fn putstr(&mut self, data: *mut u16) {
        unsafe {
            self.write(data, strlen(data));
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() {
    let mut terminal = Terminal::new();

    terminal.putchar(b'H');
    terminal.putchar(b'e');
    terminal.putchar(b'l');
    terminal.putchar(b'l');
    terminal.putchar(b'o');
    terminal.putchar(b',');
    terminal.putchar(b' ');
    terminal.putchar(b'W');
    terminal.putchar(b'o');
    terminal.putchar(b'r');
    terminal.putchar(b'l');
    terminal.putchar(b'd');
    terminal.putchar(b'!');
    loop {}
}
