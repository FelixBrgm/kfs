use core::{arch::asm, cmp, ptr::write_volatile};

#[cfg(test)]
use spin::Mutex;

#[repr(u8)]
#[allow(unused)]
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
#[allow(static_mut_refs)]
fn get_vga_buffer_ptr() -> *mut u16 {
    unsafe { VGA_BUFFER_ADDR.as_mut_ptr() }
}

pub struct OutOfBoundsError;

/// Abstraction for managing the [Text-mode cursor](https://wiki.osdev.org/Text_Mode_Cursor).
#[derive(Clone, Copy)]
pub struct Cursor {}

#[allow(unused)]
impl Cursor {
    const LOCATION_REG_LOW: u8 = 0x0F;
    const LOCATION_REG_HIGH: u8 = 0x0E;
    const REG_START: u8 = 0x0A;
    const REG_END: u8 = 0x0B;

    /// Updates the text-mode cursor position in the VGA buffer by setting the CRTC's
    /// [location registers](http://www.osdever.net/FreeVGA/vga/crtcreg.htm#0F) (`0x0F` and `0x0D`)
    /// to `x, y`.
    ///
    /// ## SAFETY
    /// 1.  This function uses `Cursor::update`, which writes directly to the VGA buffer. In user-mode, this **will** result
    ///     in invalid memory access.
    ///
    /// 2.  `update_pos` may cause undefined behavior if called with `x` or `y` values outside of the range `0x00..=0x0F`.
    pub unsafe fn update_pos(&self, x: u16, y: u16) {
        let out_of_bounds: bool = !(0..VGA_HEIGHT).contains(&(y as u8)) || !(0..VGA_WIDTH).contains(&(x as u8));
        if out_of_bounds {
            return;
        }

        let pos = y * VGA_WIDTH as u16 + x;

        self.update(Cursor::LOCATION_REG_LOW, (pos & 0xFF) as u8);
        self.update(Cursor::LOCATION_REG_HIGH, ((pos >> 8) & 0xFF) as u8);
    }

    /// Resizes the cursor by updating the [cursor end & start register](http://www.osdever.net/FreeVGA/vga/crtcreg.htm#0A)
    /// (`0x0A` and `0x0B`) to `start, end`. The values of `start` and `end` are expected to be in the range `0x00..=0x0F`.
    ///
    /// ## SAFETY
    /// 1.  This function uses `Cursor::update`, which writes directly to the VGA buffer. In user-mode, this **will** result
    ///     in invalid memory access.
    ///
    /// 2.  `resize` may cause undefined behavior if called with `start` or `end` values outside of the range `0x00..=0x0F`.
    pub unsafe fn resize(&self, start: u8, end: u8) {
        self.update(Cursor::REG_START, start);
        self.update(Cursor::REG_END, end);
    }

    /// Abstraction for the ugliness behind updating the cursor.
    ///
    /// `0x3D4` is the I/O port address for the VGA's CRTC ([Cathode-ray tube](https://en.wikipedia.org/wiki/Cathode-ray_tube))'s
    /// index register. The value being loaded into it defines which CRTC functionality we want to access.
    /// The different indices that can be loaded into it are documented [here](http://www.osdever.net/FreeVGA/vga/).
    ///
    /// After the index has been loaded into the `0x3D4`, `dx`, (where the index register is stored) can be incremented by
    /// one. This will move it to `0x3D5`, the CRTC's data register, signifying the CRTC's readiness to receive the input values.
    ///
    /// ## SAFETY:
    /// This writes to the VGA buffer directly, running this in a non-bare-metal environment
    /// will result in invalid memory access.
    unsafe fn update(&self, index: u8, value: u8) {
        asm!(
            "mov dx, 0x3D4",
            "mov al, {index}",
            "out dx, al",
            "inc dx",
            "mov al, {value}",
            "out dx, al",
            index = in(reg_byte) (index),
            value = in(reg_byte) (value),
            out("dx") _,
            out("al") _,
        )
    }
}

#[allow(unused)]
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
            t.cursor.update_pos(0, 0);
            t.cursor.resize(0x0D, 0x0F);
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

        let _ = self.write_char_at(self.y, self.x, 0);
    }

    #[allow(unused)]
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
                let _ = self.write_char_at(row, col, 0);
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
            self.cursor.update_pos(self.x as u16, self.y as u16);
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
        {
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
            self.cursor.update_pos(self.x as u16, self.y as u16);
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
            self.cursor.update_pos(self.x as u16, self.y as u16);
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
static VGA_BUFFER_LOCK: Mutex<()> = Mutex::new(());

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

        for _ in 0..VGA_WIDTH {
            v.inc_cursor();
        }

        assert_eq!(v.x, 0, "Vga::x should wrap around when reaching 64");

        v.clear_screen();
    }

    #[test]
    fn test_backspace_line_start_empty_previous_line() {
        let _guard = VGA_BUFFER_LOCK.lock();

        let mut v = Vga::new();

        v.new_line();
        v.delete_char();

        assert_eq!(v.y, 0, "Vga::y should decrease by 1 when deleting a character at the beginning of a line");
        assert_eq!(v.x, 0, "Vga::x should return to the beginning of the previous line when it is empty");

        v.clear_screen();
    }

    #[test]
    fn test_backspace_line_start_previous_line_with_content() {
        let _guard = VGA_BUFFER_LOCK.lock();

        let mut v = Vga::new();

        v.write_u8_arr(b"Hello, World");
        v.new_line();
        v.delete_char();

        assert_eq!(v.y, 0, "Vga::y should decrease by 1 when deleting a character at the beginning of a line");
        assert_eq!(
            v.x, 12,
            "Vga::x should return to the last written non-null character of the previous line when deleting a line"
        );

        v.clear_screen();
    }

    #[test]
    fn test_hello_world() {
        let _guard = VGA_BUFFER_LOCK.lock();

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

        v.clear_screen();
    }
}
