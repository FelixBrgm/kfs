#[repr(u8)]
#[derive(PartialEq, Clone, Copy)]
pub enum Granularity {
    SingleByte = 0,
    PageSize4K = 1,
}

#[derive(PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum DataProtectionSize {
    Segm16bit = 0,
    Segm32bit = 1,
}

#[derive(PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum LongMode {
    Other = 0,
    Segm64bit = 1,
}

pub struct Flag {
    granularity: Granularity,
    data_protection_size: DataProtectionSize,
    long_mode: LongMode,
}

impl Flag {
    pub fn new(
        granularity: Granularity,
        data_protection_size: DataProtectionSize,
        long_mode: LongMode,
    ) -> Self {
        if data_protection_size == DataProtectionSize::Segm32bit && long_mode == LongMode::Segm64bit
        {
            panic!("error while creating GDT Flag combination - can't have DataProtectionBit and LongModeBit both set at the same time")
        }
        Flag {
            granularity,
            data_protection_size,
            long_mode,
        }
    }

    pub fn to_u8(&self) -> u8 {
        let mut flag: u8 = 0;

        flag |= (self.granularity as u8) << 3;
        flag |= (self.data_protection_size as u8) << 2;
        flag |= (self.long_mode as u8) << 1;

        flag = flag << 4; // Because the lower 4 bits are used by limit
        flag
    }
}
