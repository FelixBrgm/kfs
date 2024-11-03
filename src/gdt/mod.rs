pub mod access;
pub mod entry;
pub mod flag;

pub use access::*;
pub use entry::*;
pub use flag::*;

#[cfg(test)]
mod test {
    // testcased from https://wiki.osdev.org/GDT_Tutorial
    use super::*;

    #[test]
    fn test_gdt_code_pl0() {
        let flag = Flag::new(
            Granularity::PageSize4K,
            DataProtectionSize::Segm32bit,
            LongMode::Other,
        );
        let access = Access::new(
            Presence::Valid,
            DescriptorPrivilege::Lvl0,
            SegmentType::CodeOrData,
            ExecutabilityType::Code,
            Direction::GrowsUp,
            ReadWriteAble::Set,
            AccessBit::OnlyForSpecial,
        );
        let entry = Entry::new(0, 0x000FFFFF, flag, access).to_u64();
        assert_eq!(entry, 0x00CF9A000000FFFF)
    }

    #[test]
    fn test_gdt_data_pl0() {
        let flag = Flag::new(
            Granularity::PageSize4K,
            DataProtectionSize::Segm32bit,
            LongMode::Other,
        );
        let access = Access::new(
            Presence::Valid,
            DescriptorPrivilege::Lvl0,
            SegmentType::CodeOrData,
            ExecutabilityType::Data,
            Direction::GrowsUp,
            ReadWriteAble::Set,
            AccessBit::OnlyForSpecial,
        );
        let entry = Entry::new(0, 0x000FFFFF, flag, access).to_u64();
        assert_eq!(entry, 0x00CF92000000FFFF)
    }

    #[test]
    fn test_gdt_code_pl3() {
        let flag = Flag::new(
            Granularity::PageSize4K,
            DataProtectionSize::Segm32bit,
            LongMode::Other,
        );
        let access = Access::new(
            Presence::Valid,
            DescriptorPrivilege::Lvl3,
            SegmentType::CodeOrData,
            ExecutabilityType::Code,
            Direction::GrowsUp,
            ReadWriteAble::Set,
            AccessBit::OnlyForSpecial,
        );
        let entry = Entry::new(0, 0x000FFFFF, flag, access).to_u64();
        assert_eq!(entry, 0x00CFFA000000FFFF)
    }

    #[test]
    fn test_gdt_data_pl3() {
        let flag = Flag::new(
            Granularity::PageSize4K,
            DataProtectionSize::Segm32bit,
            LongMode::Other,
        );
        let access = Access::new(
            Presence::Valid,
            DescriptorPrivilege::Lvl3,
            SegmentType::CodeOrData,
            ExecutabilityType::Data,
            Direction::GrowsUp,
            ReadWriteAble::Set,
            AccessBit::OnlyForSpecial,
        );
        let entry = Entry::new(0, 0x000FFFFF, flag, access).to_u64();
        assert_eq!(entry, 0x00CFF2000000FFFF)
    }
}
