static GDT: [Entry; 8192] = [Entry { descriptor: 0 }; 8192];

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Entry {
    descriptor: u64
}


/// Hey this is desc
/// 
impl Entry {
    pub fn new(base: u32, limit: u32, flag: u16) -> Self {
        let mut descriptor: u64 = 0;
        // Create the high 32 bit segment
        descriptor  =  (limit as u64       & 0x000F0000) as u64;         // set limit bits 19:16
        descriptor |= (((flag as u64) <<  8) as u64 & 0x00F0FF00) as u64;         // set type, p, dpl, s, g, d/b, l and avl fields
        descriptor |= (((base as u64) >> 16) & 0x000000FF) as u64;         // set base bits 23:16
        descriptor |=  (base as u64        & 0xFF000000) as u64;         // set base bits 31:24
    
        // Shift by 32 to allow for low part of segment
        descriptor <<= 32;
    
        // Create the low 32 bit segment
        descriptor |= ((base as u64) << 16) as u64 ;                       // set base bits 15:0
        descriptor |= ((limit as u64)  & 0x0000FFFF) as u64;
        Entry { descriptor }
    }
}

#[cfg(test)]
mod test {
    // Testcases taken from https://wiki.osdev.org/GDT_Tutorial
    use super::*;
    #[test]
    fn test_gdt_entry_null() {
        assert_eq!(Entry::new(0, 0, 0).descriptor, 0x0000000000000000);
    }
    
    #[test]
    fn test_gdt_entry_special_1 () {
        assert_eq!(Entry::new(0, 0x000FFFFF, 49394).descriptor, 0x00CFF2000000FFFF);
    }
    #[test]
    fn test_gdt_entry_special_2 () {
        assert_eq!(Entry::new(0, 0x000FFFFF, 49402).descriptor, 0x00CFFA000000FFFF);
    }
    #[test]
    fn test_gdt_entry_special_3 () {
        assert_eq!(Entry::new(0, 0x000FFFFF, 49298).descriptor, 0x00CF92000000FFFF);
    }
    #[test]
    fn test_gdt_entry_special_4 () {
        assert_eq!(Entry::new(0, 0x000FFFFF, 49306).descriptor, 0x00CF9A000000FFFF);
    }
    
}