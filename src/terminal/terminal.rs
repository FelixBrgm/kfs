use super::{
    ps2::Key,
    vga::{flush_vga, Color, Entry},
};

pub const BUFFER_SIZE: usize = 1000;

pub struct Terminal {
    pub buffer: [u16; BUFFER_SIZE],
    pub cursor: usize,
    pub last_entry_index: usize,
    pub rows_scrolled: usize
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            buffer: [Entry::new(b' ').to_u16(); BUFFER_SIZE],
            cursor: 0,
            last_entry_index: 0,
            rows_scrolled: 0
        }
    }

    pub fn handle_key(&mut self, key: Key) {
        if key as u8 <= b'z' && key as u8 >= b'a' {
            self.write(key as u8);
        } else if key == Key::Enter {
            self.write(b'\n');
        } else if key == Key::Space {
            self.write(b' ');
        } else if key == Key::Backspace {
            if self.cursor > 0 {
                self.cursor -= 1;
            }
            self.remove_entry_at(self.cursor);
        } else if key == Key::ArrowRight {
            if self.cursor < BUFFER_SIZE - 1 && self.cursor <= self.last_entry_index {
                self.cursor += 1;
            }
        } else if key == Key::ArrowLeft {
            if self.cursor > 0 {
                self.cursor -= 1;
            }
        } else if key == Key::ArrowDown {
            self.scroll(-1);
        } else if key == Key::ArrowUp {
            self.scroll(1);
        }
    }

    pub fn scroll(&mut self, delta: isize) {
        if delta >= 0 {
            self.rows_scrolled += delta as usize;
        }else if delta < 0 && delta.abs() as usize <= self.rows_scrolled {
            self.rows_scrolled -= delta.abs() as usize;
        } else {
            self.rows_scrolled = 0;
        }
    }

    pub fn write(&mut self, character: u8) {
        self.write_color(character, Color::Default as u8);
    }

    pub fn write_color(&mut self, character: u8, color: u8) {
        if self.cursor >= BUFFER_SIZE - 1 {
            return;
        }
        let mut index = BUFFER_SIZE - 2;
        while index + 1 > self.cursor {
            self.buffer[index + 1] = self.buffer[index];
            index -= 1;
        }

        self.rows_scrolled = 0;
        self.last_entry_index += 1;
        self.buffer[self.cursor] = Entry::new_with_color(character, color).to_u16();

        self.cursor += 1;
    }

    pub fn flush(&self) {
        flush_vga(self);
    }

    fn remove_entry_at(&mut self, mut index: usize) {
        while (index + 1) < BUFFER_SIZE {
            self.buffer[index] = self.buffer[index + 1];
            index += 1;
        }
        self.last_entry_index -= 1;
        self.buffer[index] = Entry::new(b' ').to_u16();
    }
}
