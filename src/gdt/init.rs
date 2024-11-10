use super::*;

pub fn init() {
    unsafe {
        GDT[0] = 0;
    }
    unsafe { GDT[1] = entry_kernel_code() }
    unsafe { GDT[2] = entry_kernel_data() }
    // unsafe { GDT[3] = entry_user_code() }
    // unsafe { GDT[4] = entry_user_data() }
}

pub fn entry_kernel_code() -> u64 {
    Entry::new(
        0,
        0xFFFFF,
        Flag::new(
            crate::gdt::Granularity::PageSize4K,
            DataProtectionSize::Segm32bit,
            LongMode::Other,
        ),
        Access::new(
            Presence::Valid,
            DescriptorPrivilege::Lvl0,
            SegmentType::CodeOrData,
            ExecutabilityType::Code,
            Direction::GrowsUp,
            ReadWriteAble::Set,
            AccessBit::OnlyForSpecial,
        ),
    )
    .to_bitmap_u64()
}
pub fn entry_kernel_data() -> u64 {
    Entry::new(
        0,
        0xFFFFF,
        Flag::new(
            crate::gdt::Granularity::PageSize4K,
            DataProtectionSize::Segm32bit,
            LongMode::Other,
        ),
        Access::new(
            Presence::Valid,
            DescriptorPrivilege::Lvl0,
            SegmentType::CodeOrData,
            ExecutabilityType::Data,
            Direction::GrowsUp,
            ReadWriteAble::Set,
            AccessBit::OnlyForSpecial,
        ),
    )
    .to_bitmap_u64()
}
pub fn entry_user_code() -> u64 {
    Entry::new(
        0,
        0xFFFFF,
        Flag::new(
            crate::gdt::Granularity::PageSize4K,
            DataProtectionSize::Segm32bit,
            LongMode::Other,
        ),
        Access::new(
            Presence::Valid,
            DescriptorPrivilege::Lvl3,
            SegmentType::CodeOrData,
            ExecutabilityType::Code,
            Direction::GrowsUp,
            ReadWriteAble::Set,
            AccessBit::OnlyForSpecial,
        ),
    )
    .to_bitmap_u64()
}
pub fn entry_user_data() -> u64 {
    Entry::new(
        0,
        0xFFFFF,
        Flag::new(
            crate::gdt::Granularity::PageSize4K,
            DataProtectionSize::Segm32bit,
            LongMode::Other,
        ),
        Access::new(
            Presence::Valid,
            DescriptorPrivilege::Lvl3,
            SegmentType::CodeOrData,
            ExecutabilityType::Data,
            Direction::GrowsUp,
            ReadWriteAble::Set,
            AccessBit::OnlyForSpecial,
        ),
    )
    .to_bitmap_u64()
}

#[cfg(test)]
mod test {

    use crate::gdt::*;
    #[test]
    pub fn test_kernel_code() {
        assert_eq!(entry_kernel_code(), 0x00CF9A000000FFFF);
    }
    #[test]
    pub fn test_kernel_data() {
        assert_eq!(entry_kernel_data(), 0x00CF92000000FFFF);
    }
    #[test]
    pub fn test_user_code() {
        assert_eq!(entry_user_code(), 0x00CFFA000000FFFF);
    }
    #[test]
    pub fn test_user_data() {
        assert_eq!(entry_user_data(), 0x00CFF2000000FFFF);
    }
}
