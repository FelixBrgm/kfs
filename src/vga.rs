#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum VGAColor {
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

pub struct Buffer {
    color: u8,
    row: usize,
    col: usize,
    buf: *mut u16,
}

fn vga_entry_color(foreground: VGAColor, background: VGAColor) -> u8 {
    (foreground as u8) | (background as u8) << 4
}

fn vga_entry(c: u8, color: u8) -> u16 {
    (c as u16) | (color as u16) << 8
}

impl Buffer {
    pub fn new() -> Self {
        let mut buffer = Buffer {
            color: vga_entry_color(VGAColor::LightGrey, VGAColor::Black),
            row: 0,
            col: 0,
            buf: 0xB8000 as *mut u16,
        };
        buffer.clear_screen();
        buffer
    }

    pub fn clear_screen(&mut self) {
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

    pub fn set_color(&mut self, color: u8) {
        self.color = color;
    }

    pub fn put_entry_at(&mut self, c: u8, col: u8, row: u8) {
        let index = row * VGA_WIDTH as u8 + col;
        unsafe { *self.buf.offset(index as isize) = vga_entry(c, self.color) }
    }

    pub fn putchar(&mut self, c: u8) {
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

    pub fn write(&mut self, data: *mut u16, size: isize) {
        for idx in 0..size {
            unsafe {
                self.putchar(*data.offset(idx) as u8);
            }
        }
    }
}
