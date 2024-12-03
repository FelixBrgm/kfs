use super::{
    ps2::Key,
    vga::{flush_vga, Entry},
};

const BUFFER_SIZE: usize = 1000;

pub struct Terminal {
    // ad a iter from viewport
    pub buffer: [u16; BUFFER_SIZE],
    pub cursor: usize,
    pub view_index: usize,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            buffer: [Entry::new(b' ').to_u16(); BUFFER_SIZE],
            cursor: 0,
            view_index: 0,
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
            self.cursor += 1;
        } else if key == Key::ArrowLeft {
            if self.cursor > 0 {
                self.cursor -= 1;
            }
        }
    }

    pub fn write(&mut self, character: u8) {
        let mut index = BUFFER_SIZE - 2;
        while index + 1 > self.cursor {
            self.buffer[index + 1] = self.buffer[index];
            index -= 1;
        }
        self.buffer[self.cursor] = Entry::new(character).to_u16();

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
        self.buffer[index] = Entry::new(b' ').to_u16();
    }
}
