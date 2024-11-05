use crate::gdt::access::Access;
use crate::gdt::flag::Flag;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Entry {
    base_31_24: u8,
    flags_limit_19_16: u8,
    access: u8,
    base_23_16: u8,
    base_15_0: u16,
    limit_15_0: u16,
}

impl Entry {
    pub fn new(base: u32, limit: u32, flag: Flag, access: Access) -> Self {
        Entry {
            base_31_24: ((base & 0xFF000000) >> 24) as u8,
            flags_limit_19_16: flag.to_u8() | ((limit & 0x000F0000) >> 16) as u8,
            access: access.to_u8(),
            base_23_16: ((base & 0x00FF0000) >> 16) as u8,
            base_15_0: (base & 0x0000FFFF) as u16,
            limit_15_0: (limit & 0x0000FFFF) as u16,
        }
    }
    pub fn new_zero() -> Self {
        Entry {
            base_31_24: 0,
            flags_limit_19_16: 0,
            access: 0,
            base_23_16: 0,
            base_15_0: 0,
            limit_15_0: 0,
        }
    }

    pub fn to_u64(&self) -> u64 {
        let mut result: u64 = 0;

        result |= (self.base_31_24 as u64) << 56;
        result |= (self.flags_limit_19_16 as u64) << 48;
        result |= (self.access as u64) << 40;
        result |= (self.base_23_16 as u64) << 32;
        result |= (self.base_15_0 as u64) << 16;
        result |= self.limit_15_0 as u64;

        result
    }
}
