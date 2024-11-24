use crate::vga::{
    buffer::{Buffer, MAX_BUFFERED_LINES},
    color::Color,
    cursor::Cursor,
};
use core::ptr::write_volatile;

mod buffer;
mod color;
mod cursor;

#[cfg(test)]
use spin::Mutex;

/// Maximum width of the [VGA buffer](https://wiki.osdev.org/Text_UI).
pub const VGA_WIDTH: u8 = 80;
/// Maximum height of the [VGA buffer](https://wiki.osdev.org/Text_UI).
pub const VGA_HEIGHT: u8 = 25;
/// Maximum amount of `u16` entries in the VGA buffer.
pub const VGA_BUFFER_SIZE: u16 = (VGA_WIDTH as u16) * (VGA_HEIGHT as u16);

#[cfg(test)]
/// Statically allocated array of size `[u16; 2000]`, used to simulate the VGA buffer in test mode.
static mut MOCK_VGA_BUFFER: [u16; VGA_WIDTH as usize * VGA_HEIGHT as usize] = [0; VGA_WIDTH as usize * VGA_HEIGHT as usize];

#[cfg(not(test))]
/// Address of the [VGA buffer](https://wiki.osdev.org/Text_UI), can be directly written to to
/// display characters in kernel-mode.
const VGA_BUFFER_ADDR: *mut u16 = 0xB8000 as *mut u16;

#[cfg(test)]
#[allow(static_mut_refs)]
/// Returns `MOCK_VGA_BUFFER` as `*mut u16` for simulations in test mode.
fn get_vga_buffer_ptr() -> *mut u16 {
    unsafe { MOCK_VGA_BUFFER.as_mut_ptr() }
}

#[allow(unused)]
#[derive(PartialEq, Eq)]
/// Represents a direction, used for moving the cursor.
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
/// Mutex used to prevent concurrent access to `MOCK_VGA_BUFFER` in tests.
static VGA_BUFFER_LOCK: Mutex<()> = Mutex::new(());

pub struct OutOfBoundsError;

#[derive(Clone, Copy)]
/// Abstraction for VGA buffer interactions.
pub struct Vga {
    color: u8,
    x: u8,
    y: u8,
    cursor: Cursor,
    buffer: Buffer,
    line_offset: u8,
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
            buffer: Buffer::new(),
            line_offset: 0,
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

    /// Moves cursor in `dir`. Supports moving up and down, though this is not enabled by default in `kernel_main`,
    /// as this is more of a text editing than terminal feature.
    /// ### Example Usage:
    /// ```
    /// let mut v = Vga::new();
    ///
    /// loop {
    ///     if let Some(c) = ps2::read_if_ready(None) {
    ///         if c == ps2::ARROW_LEFT {
    ///             v.move_cursor(Direction::Left);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn move_cursor(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.y = self.y.saturating_sub(1),
            Direction::Down => self.y = (self.y + 1).min(MAX_BUFFERED_LINES - 1),
            Direction::Left => self.x = self.x.saturating_sub(1),
            Direction::Right => self.x = self.x + self.buffer.block_length(self.x, self.y + self.line_offset) as u8,
        }

        unsafe {
            self.cursor.update_pos(self.x as u16, self.y as u16);
        }
    }

    /// Writes a character to the VGA buffer at `self.x, self.y` and increments its cursor.
    /// ### Example Usage:
    /// ```
    /// let mut v = Vga::new();
    ///
    /// v.write_char(b'6');
    /// ```
    pub fn write_char(&mut self, c: u8) {
        self.shift_text_right(self.x, 1);

        let _ = self.write_char_at(self.x, self.y, c);
        self.inc_cursor();
        self.flush();
    }

    /// Deletes the character from the VGA buffer at `self.x, self.y` and decrements the cursor.
    ///
    /// ### Example Usage:
    /// ```
    /// let mut v = Vga::new();
    ///
    /// loop {
    ///     if let Some(char) = ps2::read_if_ready(None) == ps2::BACKSPACE {
    ///         v.delete_char();
    ///     }
    /// }
    /// ```
    pub fn delete_char(&mut self) {
        self.dec_cursor();

        let _ = self.write_char_at(self.x, self.y, 0);
        self.flush();
    }

    #[allow(unused)]
    /// Writes `s` to the VGA buffer starting at `self.x, self.y` and increments the cursor by `s.len()`.
    ///
    /// ### Example Usage
    /// ```
    /// let mut v = Vga::new();
    ///
    /// v.write_u8_arr(b"Hello, World!");
    /// ```
    pub fn write_u8_arr(&mut self, s: &[u8]) {
        for c in s.iter() {
            if *c == 0 {
                return;
            }
            self.write_char(*c);
        }
        self.flush();
    }

    /// Fills the whole VGA buffer with `0u16`, clearing the screen. This should generally be used
    /// at the beginning of `kernel_main` to clear out leftover output from Grub.
    /// ### Example Usage:
    /// ```
    /// let mut v = Vga::new();
    ///
    /// v.clear_screen();
    /// ```
    pub fn clear_screen(&mut self) {
        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                let _ = self.write_char_at(x, y, 0);
            }
        }
        self.flush();
    }

    /// Moves `self.y` to `self.y + 1` and `self.x` to `0`, and updates the cursor.
    /// ### Example Usage:
    /// ```
    /// let mut v = Vga::new();
    ///
    /// loop {
    ///     if let Some(c) = ps2::read_if_ready(None) {
    ///         if c == ps2::ENTER {
    ///             v.new_line();
    ///         }
    ///     }
    /// }
    pub fn new_line(&mut self) {
        let reached_buffer_limit = self.y == VGA_HEIGHT - 1 && self.line_offset == (MAX_BUFFERED_LINES - VGA_HEIGHT);
        if reached_buffer_limit {
            return;
        }

        let block_length = self.buffer.block_length(0, self.y + self.line_offset);
        let x = block_length % VGA_WIDTH as u16;
        let y = self.y as u16 + self.line_offset as u16;
        let _ = self.write_char_at(x as u8, y as u8, Buffer::NEWLINE);

        self.x = 0;
        self.y = self.y.saturating_add(1);

        if self.y >= VGA_HEIGHT {
            self.scroll_down();
        }

        #[cfg(not(test))]
        unsafe {
            self.cursor.update_pos(self.x as u16, self.y as u16);
        }

        self.flush();
    }

    /// Sets the 4 most significant bits of `self.color` to `foreground`, setting the
    /// font color of the VGA buffer.
    ///
    /// ### Example Usage:
    /// ```
    /// let mut v = Vga::new();
    ///
    /// v.set_foreground_color(Color::White);
    /// ```
    pub fn set_foreground_color(&mut self, foreground: Color) {
        self.color &= 0xF0;
        self.color |= foreground.to_foreground();
    }

    /// Sets the 4 least significant bits of `self.color` to `background`, setting the
    /// background color of the VGA buffer.
    /// ### Example Usage:
    /// ```
    /// let mut v = Vga::new();
    ///
    /// v.set_background_color(Color::Black);
    /// ```
    pub fn set_background_color(&mut self, background: Color) {
        self.color &= 0x0F;
        self.color |= background.to_background();
    }

    /// Shifts the text back if a write is happening in the middle of a text block. Else, does nothing.
    ///
    /// Needs to be called at every write.
    fn shift_text_right(&mut self, from_x: u8, by: u8) {
        let block_length = self.buffer.block_length(from_x, self.y + self.line_offset);

        for x in (from_x..(from_x + block_length as u8)).rev() {
            let x_shifted = (x + by) % VGA_WIDTH;
            let y_shifted = self.y + if (x + by) >= VGA_WIDTH { 1 } else { 0 };

            let current_char = self.buffer.at(self.y as u16 * VGA_WIDTH as u16 + x as u16).unwrap();
            let _ = self.write_char_at(x_shifted, y_shifted, (*current_char & 0xFF) as u8);
        }
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

    /// Flushes `self.buffer.slice(self.line_offset)` into the VGA buffer, writing it to the console.
    fn flush(&self) {
        let current_displayed_content = self.buffer.slice(self.line_offset);

        for (idx, &entry) in current_displayed_content.iter().enumerate() {
            unsafe {
                write_volatile(self.get_buffer_addr().add(idx), entry);
            }
        }
    }

    /// Writes `character` at `self.x == x` and `self.y == y` into the VGA buffer.
    fn write_char_at(&mut self, x: u8, y: u8, character: u8) -> Result<(), OutOfBoundsError> {
        if y == VGA_HEIGHT - 1 && x == VGA_WIDTH - 1 {
            return Ok(());
        }

        if y >= VGA_HEIGHT || x >= VGA_WIDTH {
            return Err(OutOfBoundsError);
        }

        let entry: u16 = (character as u16) | (self.color as u16) << 8;
        let index: u16 = (y as u16 + self.line_offset as u16) * VGA_WIDTH as u16 + x as u16;

        self.buffer.write(0, index, entry);

        Ok(())
    }

    /// Decrements the cursor, taking line wrapping into account.
    fn dec_cursor(&mut self) {
        let on_first_col = self.x == 0;
        let on_first_row = self.y == 0;

        if on_first_col && on_first_row {
            self.line_offset = self.line_offset.saturating_sub(1);
            self.x = self.get_x_for_y(self.y as usize) as u8;
        } else if on_first_col {
            self.y = self.y.saturating_sub(1);
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
        if self.x == VGA_WIDTH - 1 && self.y == VGA_HEIGHT - 1 {
            return;
        }

        self.x = self.x.saturating_add(1);

        if self.x >= VGA_WIDTH {
            self.x = 0;
            self.y = self.y.saturating_add(1);
        }

        if self.y >= VGA_HEIGHT {
            self.scroll_down();
        }

        #[cfg(not(test))]
        unsafe {
            self.cursor.update_pos(self.x as u16, self.y as u16);
        }
    }

    fn scroll_down(&mut self) {
        if self.line_offset == MAX_BUFFERED_LINES - VGA_HEIGHT {
            return;
        }

        self.line_offset = (self.line_offset + 1).min(MAX_BUFFERED_LINES - VGA_HEIGHT);
        self.y = VGA_HEIGHT - 1;

        for x in 0..VGA_WIDTH {
            let _ = self.write_char_at(x, self.y, 0);
        }

        self.flush();
    }

    /// Gets the position of the last written character for `y`, to ensure the cursor returns
    /// to the correct position when backspacing at `x == 0`.
    fn get_x_for_y(&self, y: usize) -> usize {
        self.buffer.block_length(0, (y + self.line_offset as usize) as u8).saturating_sub(1) as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_vga() {
        let _guard = VGA_BUFFER_LOCK.lock();

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
        let _guard = VGA_BUFFER_LOCK.lock();

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
            let buf = &MOCK_VGA_BUFFER[0..12];

            let mut written_content: [u8; 12] = [0u8; 12];

            for (idx, &entry) in buf.iter().enumerate() {
                written_content[idx] = (entry & 0x00FF) as u8;
            }

            assert_eq!(&written_content, b"Hello, World", "Content has not been written to VGA_BUFFER_ADDR");
        }

        v.clear_screen();
    }

    #[test]
    fn test_newline_on_first_line() {
        let _guard = VGA_BUFFER_LOCK.lock();

        let mut v = Vga::new();

        v.write_u8_arr(b"Hello, World");
        v.new_line();

        assert_eq!(
            (v.buffer.at(0).unwrap() & 0xFF) as u8,
            b'H',
            "First character of the previous line should not be deleted when pressing enter"
        );

        v.clear_screen();
    }

    #[test]
    fn test_2_newlines_on_first_line() {
        let _guard = VGA_BUFFER_LOCK.lock();

        let mut v = Vga::new();

        v.write_u8_arr(b"Hello, World");
        v.new_line();
        v.new_line();

        assert_eq!(
            (v.buffer.at(0).unwrap() & 0xFF) as u8,
            b'H',
            "First charactqer of the previous line should not be deleted when pressing enter"
        );

        v.clear_screen();
    }

    #[test]
    fn test_block_length_first_line() {
        let _guard = VGA_BUFFER_LOCK.lock();

        let mut v = Vga::new();

        v.write_u8_arr(b"Hello, World");

        assert_eq!(
            v.buffer.block_length(0, v.y + v.line_offset),
            12,
            "First character of the previous line should not be deleted when pressing enter"
        );

        v.clear_screen();
    }

    #[test]
    fn test_newline_underflow() {
        let _guard = VGA_BUFFER_LOCK.lock();

        let mut v = Vga::new();

        v.write_u8_arr(b"Hello, World");

        v.new_line();

        let block_length = v.buffer.block_length(0, v.y + v.line_offset);
        let x = block_length % VGA_WIDTH as u16;
        let y = v.y as u16 + v.line_offset as u16;

        assert_eq!(x, 0, "Vga::x should equal to zero here");
        assert_eq!(y, 1, "Vga::x should equal to one here");
    }
}
