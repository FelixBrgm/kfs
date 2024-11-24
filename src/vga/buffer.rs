use crate::vga::{VGA_BUFFER_SIZE, VGA_WIDTH};

/// Amount of lines buffered in `Buffer`.
pub const MAX_BUFFERED_LINES: u8 = 100;

#[derive(Clone, Copy)]
/// Buffer implementation for storing content beyond the VGA buffer size of 4000 bytes (80 x 25
/// u16 entries).
///
/// Intended to be used in the `Vga` implementation. Can be written to, and flushed into the VGA buffer
/// using `Vga::flush`.
pub struct Buffer {
    buf: [u16; (VGA_WIDTH as usize) * (MAX_BUFFERED_LINES as usize)],
}

impl Buffer {
    /// Represents a newline (`\n` makes a weird character, `0xFF` just does not print anything
    /// while still being differentiable from `0x00`).
    pub const NEWLINE: u8 = 0xFF;

    /// Returns a new `Buffer` object with a buffer size of `VGA_WIDTH * MAX_BUFFERED_LINES`. `VGA_WIDTH`
    /// is fixed to 80 (hardware limitation), and `MAX_BUFFERED_LINES` can be increased freely as long as enough memory
    /// is available.
    pub fn new() -> Self {
        Self {
            buf: [0u16; (VGA_WIDTH as usize) * (MAX_BUFFERED_LINES as usize)],
        }
    }

    /// Writes `entry` to `self.buf[line_offset * VGA_WIDTH + rel_index]`.
    /// ### Note
    /// This does **not** write to the VGA buffer, only to the internal one. Writes to the VGA buffer are to be handled
    /// by `Vga::flush`.
    /// ### Usage Example:
    /// ```
    /// let mut v = Vga::new();
    ///
    /// v.buffer.write(v.line_offset, 20, 0);
    /// ```
    pub fn write(&mut self, line_offset: u8, rel_index: u16, entry: u16) {
        let abs_index: usize = (line_offset as usize * VGA_WIDTH as usize) + rel_index as usize;

        if abs_index as u16 >= self.len() {
            return;
        }

        self.buf[abs_index] = entry;
    }

    /// Returns a `self.buf` slice of `VGA_BUFFER_SIZE` starting at `line_offset`.
    pub fn slice(&self, line_offset: u8) -> &[u16] {
        let start = (line_offset as usize) * (VGA_WIDTH as usize);
        let end = start + VGA_BUFFER_SIZE as usize;

        &self.buf[start..end]
    }

    pub fn at(&self, pos: u16) -> Option<&u16> {
        self.buf.get(pos as usize)
    }

    /// Returns the length of the written content starting from `from_x, from_y`, until either
    /// the next newline, or if no newline is found, until the next null VGA entry, i.e the next
    /// entry for which `(x & 0xFF) == 0` is true.
    pub fn block_length(&self, from_x: u8, from_y: u8) -> u16 {
        let slice = &self.buf[from_y as usize * VGA_WIDTH as usize + from_x as usize..];
        if let Some(ind) = slice.iter().position(|x| *x == Buffer::NEWLINE as u16) {
            return ind as u16;
        }
        slice.iter().position(|x| (*x & 0xFF) == 0).unwrap() as u16
    }

    pub fn len(&self) -> u16 {
        self.buf.len() as u16
    }
}
