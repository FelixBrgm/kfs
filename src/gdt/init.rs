use super::{Access, Entry, Flag, GDT};

pub fn init() {
    unsafe {
        GDT[0] = 0;
    }
    let kernel_code: Entry = Entry::new(
        0,
        0xFFFFF,
        Flag::new(
            super::Granularity::PageSize4K,
            super::DataProtectionSize::Segm32bit,
            super::LongMode::Other,
        ),
        Access::new(
            super::Presence::Valid,
            super::DescriptorPrivilege::Lvl0,
            super::SegmentType::CodeOrData,
            super::ExecutabilityType::Code,
            super::Direction::GrowsUp,
            super::ReadWriteAble::Clear,
            super::AccessBit::Default,
        ),
    );
    unsafe { GDT[1] = kernel_code.to_bitmap_u64() }
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    pub fn test_123() {
        let kernel_code: Entry = Entry::new(
            0,
            0xFFFFF,
            crate::gdt::Flag::new(
                crate::gdt::Granularity::PageSize4K,
                crate::gdt::DataProtectionSize::Segm32bit,
                crate::gdt::LongMode::Other,
            ),
            crate::gdt::Access::new(
                crate::gdt::Presence::Valid,
                crate::gdt::DescriptorPrivilege::Lvl0,
                crate::gdt::SegmentType::CodeOrData,
                crate::gdt::ExecutabilityType::Code,
                crate::gdt::Direction::GrowsUp,
                crate::gdt::ReadWriteAble::Clear,
                crate::gdt::AccessBit::Default,
            ),
        );
        assert_eq!(kernel_code.to_bitmap_u64(), 0);
    }
}
