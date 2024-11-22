use core::{arch::asm, cmp, error::Error};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Color {
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

impl Color {
    pub fn to_foreground(self) -> u8 {
        self as u8
    }

    pub fn to_background(self) -> u8 {
        (self as u8) << 4
    }
}

pub const VGA_WIDTH: u8 = 80;
pub const VGA_HEIGHT: u8 = 25;
const VGA_BUFFER_ADDR: *mut u16 = 0xB8000 as *mut u16;

pub struct OutOffBoundsError;

#[derive(Clone, Copy)]
pub struct Vga {
    color: u8,
    x: u8,
    y: u8,
}

impl Default for Vga {
    fn default() -> Self {
        Self::new()
    }
}

impl Vga {
    pub fn new() -> Self {
        let mut t = Vga {
            color: 0,
            x: 0,
            y: 0,
        };
        t.set_foreground_color(Color::White);
        t.set_background_color(Color::Black);
        t.update_cursor(0, 0);
        t
    }

    pub fn write_char(&mut self, c: u8) {
        let _ = self.write_char_at(self.y, self.x, c);
        self.inc_cursor();
    }

    pub fn delete_char(&mut self) {
        self.dec_cursor();
        self.write_char_at(self.y, self.x, 0);
    }

    fn write_char_at(&self, row: u8, column: u8, character: u8) -> Result<(), OutOffBoundsError> {
        if row >= VGA_HEIGHT || column >= VGA_WIDTH {
            return Err(OutOffBoundsError);
        }
        let entry: u16 = (character as u16) | (self.color as u16) << 8;
        let index: isize = row as isize * VGA_WIDTH as isize + column as isize;
        // Safety:
        // The if statement above ensures that the index never goes out of the dedicated memory for the chars on screen
        unsafe {
            *VGA_BUFFER_ADDR.offset(index) = entry;
        }
        Ok(())
    }

    pub fn write_u8_arr(&mut self, s: &[u8]) {
        for c in s.iter() {
            if *c == 0 {
                return;
            }
            self.write_char(*c);
        }
    }

    pub fn clear_screen(&mut self) {
        for row in 0..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                self.write_char_at(row, col, b' ');
            }
        }
    }

    fn update_cursor(&self, x: usize, y: usize) {
        assert!((x as u8) < VGA_WIDTH);
        assert!((y as u8) < VGA_HEIGHT);

        let pos = y * VGA_WIDTH as usize + x;

        unsafe {
            asm!(
                "mov dx, 0x3D4",
                "mov al, 0x0F",
                "out dx, al",
                "inc dx",
                "mov al, {low}",
                "out dx, al",
                "mov dx, 0x3D4",
                "mov al, 0x0E",
                "out dx, al",
                "inc dx",
                "mov al, {high}",
                "out dx, al",
                low = in(reg_byte) (pos & 0xFF) as u8,
                high = in(reg_byte) ((pos >> 8) & 0xFF) as u8,
                out("dx") _,
                out("al") _,
            );
        }
    }

    fn dec_cursor(&mut self) {
        let on_first_col = self.x == 0;
        let on_first_row = self.y == 0;

        if on_first_col && on_first_row {
            return;
        } else if on_first_col {
            self.x = VGA_WIDTH - 1;
            self.y = cmp::max(self.y - 1, 0);
        } else {
            self.x -= 1;
        }
        self.update_cursor(self.x as usize, self.y as usize);
    }

    fn inc_cursor(&mut self) {
        self.x += 1;
        if self.x >= VGA_WIDTH {
            self.x = 0;
            self.y += 1;
        }
        if self.y >= VGA_HEIGHT {
            self.y = 0;
        }
        self.update_cursor(self.x as usize, self.y as usize);
    }

    pub fn new_line(&mut self) {
        let current_row = self.y;
        while current_row == self.y {
            self.inc_cursor();
        }
        self.update_cursor(self.x as usize, self.y as usize);
    }

    pub fn set_foreground_color(&mut self, foreground: Color) {
        self.color &= 0xF0;
        self.color |= foreground.to_foreground();
    }

    pub fn set_background_color(&mut self, background: Color) {
        self.color &= 0x0F;
        self.color |= background.to_background();
    }
}
