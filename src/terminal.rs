use core::{alloc, arch::asm, cmp, error::Error, ptr::write_volatile};

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

#[cfg(test)]
static mut VGA_BUFFER_ADDR: [u16; VGA_WIDTH as usize * VGA_HEIGHT as usize] = [0; VGA_WIDTH as usize * VGA_HEIGHT as usize];

#[cfg(not(test))]
const VGA_BUFFER_ADDR: *mut u16 = 0xB8000 as *mut u16;

#[cfg(test)]
fn get_vga_buffer_ptr() -> *mut u16 {
    unsafe { VGA_BUFFER_ADDR.as_mut_ptr() }
}

pub struct OutOfBoundsError;

/// Abstraction for the ugliness behind updating the cursor.
///
/// https://wiki.osdev.org/Text_Mode_Cursor
#[derive(Clone, Copy)]
pub struct Cursor {}

impl Cursor {
    /// Updates the text-mode cursor to position `x, y` in the VGA buffer.
    /// ## SAFETY
    /// This writes to kernel-managed memory, running this in a non-bare-metal environment
    /// will result in invalid memory access.
    pub unsafe fn update(&self, x: usize, y: usize) {
        let out_of_bounds: bool = !(0..VGA_HEIGHT).contains(&(y as u8)) || !(0..VGA_WIDTH).contains(&(x as u8));
        if out_of_bounds {
            return;
        }

        let pos = y * VGA_WIDTH as usize + x;

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

#[derive(Clone, Copy)]
/// Abstraction for VGA buffer interactions.
pub struct Vga {
    color: u8,
    x: u8,
    y: u8,
    cursor: Cursor,
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
            cursor: Cursor {},
        };

        t.set_foreground_color(Color::White);
        t.set_background_color(Color::Black);

        #[cfg(not(test))]
        unsafe {
            t.cursor.update(0, 0);
        }

        t
    }

    /// Writes a character to the VGA buffer at `self.x, self.y` and increments its cursor.
    pub fn write_char(&mut self, c: u8) {
        let _ = self.write_char_at(self.y, self.x, c);
        self.inc_cursor();
    }

    /// Deletes the character from the VGA buffer at `self.x, self.y` and decrements the cursor.
    pub fn delete_char(&mut self) {
        self.dec_cursor();

        self.write_char_at(self.y, self.x, 0);
    }

    /// Writes `s` to the VGA buffer starting at `self.x, self.y` and increments the cursor by `s.len()`.
    pub fn write_u8_arr(&mut self, s: &[u8]) {
        for c in s.iter() {
            if *c == 0 {
                return;
            }
            self.write_char(*c);
        }
    }

    /// Fills the whole VGA buffer with `0u16`, clearing the screen.
    pub fn clear_screen(&mut self) {
        for row in 0..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                self.write_char_at(row, col, 0);
            }
        }
    }

    /// Moves `self.y` to `self.y + 1` and `self.x` to `0`, and updates the cursor.
    pub fn new_line(&mut self) {
        let current_row = self.y;
        while current_row == self.y {
            self.inc_cursor();
        }

        #[cfg(not(test))]
        unsafe {
            self.cursor.update(self.x as usize, self.y as usize);
        }
    }

    /// Sets the 4 most significant bits of `self.color` to `foreground`, setting the
    /// font color of the VGA buffer.
    pub fn set_foreground_color(&mut self, foreground: Color) {
        self.color &= 0xF0;
        self.color |= foreground.to_foreground();
    }

    /// Sets the 4 least significant bits of `self.color` to `background`, setting the
    /// background color of the VGA buffer.
    pub fn set_background_color(&mut self, background: Color) {
        self.color &= 0x0F;
        self.color |= background.to_background();
    }

    /// Abstraction around the VGA buffer address to avoid invalid memory access when running
    /// in test mode, where we do not have direct access to the VGA buffer.
    fn get_buffer_addr(&self) -> *mut u16 {
        #[cfg(test)]
        unsafe {
            get_vga_buffer_ptr()
        }

        #[cfg(not(test))]
        {
            VGA_BUFFER_ADDR
        }
    }

    /// Writes `character` at `self.x == x` and `self.y == y` into the VGA buffer.
    fn write_char_at(&self, y: u8, x: u8, character: u8) -> Result<(), OutOfBoundsError> {
        if y >= VGA_HEIGHT || x >= VGA_WIDTH {
            return Err(OutOfBoundsError);
        }
        let entry: u16 = (character as u16) | (self.color as u16) << 8;
        let index: isize = y as isize * VGA_WIDTH as isize + x as isize;

        // Safety:
        // The if statement above ensures that the index never goes out of the dedicated memory for the chars on screen.
        unsafe {
            write_volatile(self.get_buffer_addr().offset(index), entry);
        }
        Ok(())
    }

    /// Decrements the cursor, taking line wrapping into account.
    fn dec_cursor(&mut self) {
        let on_first_col = self.x == 0;
        let on_first_row = self.y == 0;

        if on_first_col && on_first_row {
            return;
        } else if on_first_col {
            self.y = cmp::max(self.y - 1, 0);
            self.x = self.get_x_for_y(self.y as usize) as u8;
        } else {
            self.x -= 1;
        }

        #[cfg(not(test))]
        unsafe {
            self.cursor.update(self.x as usize, self.y as usize);
        }
    }

    /// Increments the cursor, taking line wrapping into account.
    fn inc_cursor(&mut self) {
        self.x += 1;
        if self.x >= VGA_WIDTH {
            self.x = 0;
            self.y += 1;
        }
        if self.y >= VGA_HEIGHT {
            self.y = 0;
        }

        #[cfg(not(test))]
        unsafe {
            self.cursor.update(self.x as usize, self.y as usize);
        }
    }

    /// Gets the position of the last written character for `row`, to ensure the cursor returns
    /// to the correct position when backspacing at `col == 0`.
    fn get_x_for_y(&self, y: usize) -> usize {
        if !(0..VGA_HEIGHT).contains(&(y as u8)) {
            return 0;
        }

        let mut pos: isize = y as isize * VGA_WIDTH as isize + (VGA_WIDTH as isize - 1);

        // Safety:
        // The above check ensures we stay within the bounds of the VGA buffer row-wise. Then,
        // the loop condition ensures we never read outside the bounds of the current row.
        unsafe {
            while pos > y as isize * VGA_WIDTH as isize {
                if (*self.get_buffer_addr().offset(pos) & 0xFF) != 0 {
                    return pos as usize % VGA_WIDTH as usize + 1;
                }

                pos -= 1;
            }
        }

        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_vga() {
        let v = Vga::new();

        assert_eq!(v.x, 0, "Vga::x should be initialized to 0");
        assert_eq!(v.y, 0, "Vga::x should be initialized to 0");

        let expected_color = Color::Black.to_background() | Color::White.to_foreground();
        assert_eq!(
            v.color, expected_color,
            "Vga::color should be initialized to Color::Black.to_background() | Color::White.to_foreground()"
        );
    }

    #[test]
    fn test_line_wrap() {
        let mut v = Vga::new();

        for col in 0..VGA_WIDTH {
            v.inc_cursor();
        }

        assert_eq!(v.x, 0, "Vga::x should wrap around when reaching 64");
    }

    #[test]
    fn test_backspace_line_start_empty_previous_line() {
        let mut v = Vga::new();

        v.new_line();
        v.delete_char();

        assert_eq!(v.y, 0, "Vga::y should decrease by 1 when deleting a character at the beginning of a line");
        assert_eq!(v.x, 0, "Vga::x should return to the beginning of the previous line when it is empty");
    }

    #[test]
    fn test_backspace_line_start_previous_line_with_content() {
        let mut v = Vga::new();

        v.write_u8_arr(b"Hello, World");
        v.new_line();
        v.delete_char();

        assert_eq!(v.y, 0, "Vga::y should decrease by 1 when deleting a character at the beginning of a line");
        assert_eq!(
            v.x, 12,
            "Vga::x should return to the last written non-null character of the previous line when deleting a line"
        );
    }

    #[test]
    fn test_hello_world() {
        let mut v = Vga::new();

        v.write_u8_arr(b"Hello, World");
        unsafe {
            let buf = &VGA_BUFFER_ADDR[0..12];

            let mut written_content: [u8; 12] = [0u8; 12];

            for (idx, &entry) in buf.iter().enumerate() {
                written_content[idx] = (entry & 0x00FF) as u8;
            }

            assert_eq!(&written_content, b"Hello, World", "Content has not been written to VGA_BUFFER_ADDR");
        }
    }
}
