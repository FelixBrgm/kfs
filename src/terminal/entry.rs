struct Entry {
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
