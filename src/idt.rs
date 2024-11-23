#![allow(unused)]

// Do not move these, around, the order matters :)
// https://wiki.osdev.org/Interrupt_Descriptor_Table
#[repr(C, packed)]
pub struct Entry {
    offset_1: u16,       // offset bits 0..15
    selector: u16,       // code segment selector
    zero: u8,            // unused, set to 0
    type_attributes: u8, // gate type, dpl and p fields
    offset_2: u16,       // offset bits 16.31
}

impl Entry {
    pub fn new(offset_1: u16, selector: u16, type_attributes: u8, offset_2: u16) -> Self {
        Self {
            offset_1,
            selector,
            zero: 0u8,
            type_attributes,
            offset_2,
        }
    }
}

pub fn init() {}
