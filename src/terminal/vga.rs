use core::ptr::{read_volatile, write_volatile};

use crate::print::u64_to_base;

use super::{cursor::Cursor, terminal::Terminal};

pub const VIEW_WIDTH: usize = 80;
pub const VIEW_HEIGHT: usize = 25;
pub const VIEW_BUFFER_SIZE: usize = VIEW_WIDTH * VIEW_HEIGHT;
const VGA_BUFFER_ADDR: *mut u16 = 0xB8000 as *mut u16;

pub fn flush_vga(t: &Terminal) {
    let mut view_padding_whitespace: usize = 0;
    let mut counter = 10;
    for (relative_index, &entry) in t.buffer.iter().skip(t.view_start_index).enumerate() {
        let padded_relative_index = relative_index + view_padding_whitespace;

        let index_after_viewport = padded_relative_index >= VIEW_BUFFER_SIZE;
        if index_after_viewport {
            return;
        }

        match (entry & 0xFF) as u8 {
            b'\n' => {
                let padding = VIEW_WIDTH - (padded_relative_index % VIEW_WIDTH) - 1;
                view_padding_whitespace += padding;

                for i in 0..padding {
                    write_entry_to_vga(padded_relative_index + i, Entry::new(b' ').to_u16()).unwrap();
                }
            }
            _ => write_entry_to_vga(padded_relative_index, entry).unwrap(),
        }

        let relative_cursor = t.cursor - t.view_start_index;
        let padded_relative_cursor = relative_cursor + view_padding_whitespace;
        if relative_cursor == relative_index {
            unsafe {
                let c = Cursor {};
                c.update_pos(
                    (padded_relative_cursor as usize % VIEW_WIDTH) as u16,
                    (padded_relative_cursor as usize / VIEW_WIDTH) as u16,
                )
            };
        }
    }
}

#[derive(Debug)]
pub struct OutOfBoundsErr;

fn write_entry_to_vga(index: usize, entry: u16) -> Result<(), OutOfBoundsErr> {
    if index >= VIEW_BUFFER_SIZE {
        return Err(OutOfBoundsErr);
    }

    let written_entry = read_entry_from_vga(index).unwrap(); // Have to think about how we want to handle this
    if entry == written_entry {
        return Ok(());
    }

    unsafe { write_volatile(VGA_BUFFER_ADDR.add(index), entry) }
    Ok(())
}

fn read_entry_from_vga(index: usize) -> Result<u16, OutOfBoundsErr> {
    if index >= VIEW_BUFFER_SIZE {
        return Err(OutOfBoundsErr);
    }
    let e: u16 = unsafe { read_volatile(VGA_BUFFER_ADDR.add(index)) };
    Ok(e)
}

pub struct Entry {
    color: u8,
    character: u8,
}

impl Entry {
    pub fn new(character: u8) -> Self {
        Entry {
            color: Color::Default as u8,
            character,
        }
    }

    pub fn to_u16(&self) -> u16 {
        ((self.color as u16) << 8) | (self.character as u16)
    }
}

#[repr(u8)]
enum Color {
    Default = 0x07,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn testing() {
        let v = [0, 1, 2, 3, 4];
        for (i, e) in v.iter().skip(2).enumerate() {}
    }
}
