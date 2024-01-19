use bitflags::bitflags;

/// Define bits for access flag for SegmentDescriptor.
bitflags! {
    /// DescriptorFlags are shared among all descriptor types
    pub struct DescriptorFlags: u8 {
        const LONG_MODE_CODE_SEGMENT = 1 << 1;
        const PROTECTED_MODE_SEGMENT = 1 << 2;
        const PAGE_BLOCKS = 1 << 3;
    } 

    /// SegmentDescriptorFlags defines and implements all of the flags used in a SegmentDescriptor.
    pub struct SegmentDescriptorFlags: u8 {
        const ACCESSED = 1 << 0;

        // Contains some duplicate flags for semantic correctness.
        const CODE_SEGMENT_READABLE = 1 << 1;
        const DATA_SEGMENT_WRITABLE = 1 << 1;
        const DATA_SEGMENT_DOWN = 1 << 2;
        const CODE_SEGMENT_CONFORMING = 1 << 2;

        const EXECUTABLE = 1 << 3;
        const CODE_DATA_SEGMENT = 1 << 4;
        const DPL_0 = 0 << 5;
        const DPL_3 = 3 << 5;
        const PRESENT = 1 << 7;
    }

    /// SystemSegmentDescriptorFlags defines and implements all of the flags used in a SystemSegmentDescriptor.
    pub struct SystemSegmentDescriptorAccessByte: u8 {
        // 32-bit mode
        const TSS_16_AVAILABLE = 1 << 0;
        const LDT = 2 << 0;
        const TSS_16_BUSY = 3 << 0;
        const TSS_32_AVAILABLE = 9 << 0;
        const TSS_32_BUSY = 9 << 0;
    
        // Long mode
        const TSS_64_AVAILABLE = 9 << 0;
        const TSS_64_BUSY = 11 << 0;
    }
}

#[repr(C, packed)]
#[derive(Debug, PartialEq)]
/// The SegmentDescriptor is a data structure in a GDT or LDT that provides the processor with the size and location of a segment, as well as access control and status information (Intel Manual 3 3.4.5). 
struct SegmentDescriptor {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access_byte: u8,
    limit_flags: u8,
    base_high: u8,
}

impl SegmentDescriptor {
    pub fn null_descriptor() -> Self {
        Self {
            limit_low: 0,
            base_low: 0,
            base_mid: 0,
            access_byte: 0,
            limit_flags: 0,
            base_high: 0,
        }
    }

    pub fn kernel_mode_code_segment() -> Self {
        Self {
            limit_low: 0xFFFF,
            base_low: 0,
            base_mid: 0,
            access_byte: {
                SegmentDescriptorFlags::PRESENT.bits() | 
                SegmentDescriptorFlags::DPL_0.bits() |
                SegmentDescriptorFlags::CODE_DATA_SEGMENT.bits() |
                SegmentDescriptorFlags::EXECUTABLE.bits() |
                SegmentDescriptorFlags::CODE_SEGMENT_READABLE.bits()
            },
            limit_flags: {
                (DescriptorFlags::PAGE_BLOCKS.bits() | 
                DescriptorFlags::LONG_MODE_CODE_SEGMENT.bits()) << 4 | 0xF
            },
            base_high: 0
        }
    }

    pub fn kernel_mode_data_segment() -> Self {
        Self {
            limit_low: 0xFFFF,
            base_low: 0,
            base_mid: 0,
            access_byte: {
                SegmentDescriptorFlags::PRESENT.bits() | 
                SegmentDescriptorFlags::DPL_0.bits() |
                SegmentDescriptorFlags::CODE_DATA_SEGMENT.bits() |
                SegmentDescriptorFlags::DATA_SEGMENT_WRITABLE.bits()
            },
            limit_flags: {
                (DescriptorFlags::PAGE_BLOCKS.bits() | 
                DescriptorFlags::PROTECTED_MODE_SEGMENT.bits()) << 4 | 0xF
            },
            base_high: 0
        }
    }

    pub fn user_mode_code_segment() -> Self {
        Self {
            limit_low: 0xFFFF,
            base_low: 0,
            base_mid: 0,
            access_byte: {
                SegmentDescriptorFlags::PRESENT.bits() | 
                SegmentDescriptorFlags::DPL_3.bits() |
                SegmentDescriptorFlags::CODE_DATA_SEGMENT.bits() |
                SegmentDescriptorFlags::EXECUTABLE.bits() |
                SegmentDescriptorFlags::CODE_SEGMENT_READABLE.bits()
            },
            limit_flags: {
                (DescriptorFlags::PAGE_BLOCKS.bits() | 
                DescriptorFlags::LONG_MODE_CODE_SEGMENT.bits()) << 4 | 0xF
            },
            base_high: 0
        }
    }

    pub fn user_mode_data_segment() -> Self {
        Self {
            limit_low: 0xFFFF,
            base_low: 0,
            base_mid: 0,
            access_byte: {
                SegmentDescriptorFlags::PRESENT.bits() | 
                SegmentDescriptorFlags::DPL_3.bits() |
                SegmentDescriptorFlags::CODE_DATA_SEGMENT.bits() |
                SegmentDescriptorFlags::DATA_SEGMENT_WRITABLE.bits()
            },
            limit_flags: {
                (DescriptorFlags::PAGE_BLOCKS.bits() | 
                DescriptorFlags::PROTECTED_MODE_SEGMENT.bits()) << 4 | 0xF
            },
            base_high: 0
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, PartialEq)]
/// The SystemSegmentDescriptor is a data structure in a GDT or LDT that fall into two categories: system-segment descriptors and gate descriptors. System-segment descriptors point to system segment (LDT and TSS). Gate descriptors are in themselves "gates," which hold pointers to procedure entry points in code segments or hold segments desciptors for TSS's (task gates) (Intel Manual 3 3.5).
struct SystemSegmentDescriptor {
    limit_low: u16,
    base_low: u16,
    base_mid_low: u8,
    access_byte: u8,
    limit_flags: u8,
    base_mid_high: u8,
    base_high: u32,
    _reserved: u32,
}

impl SystemSegmentDescriptor {
    pub fn tss() -> Self {
        todo!();
    }
}

#[test_case]
fn test_SegmentDescriptor_null_descriptor() {
    let expected = SegmentDescriptor {
        limit_low: 0,
        base_low: 0,
        base_mid: 0,
        access_byte: 0,
        limit_flags: 0,
        base_high: 0,
    };

    let actual = SegmentDescriptor::null_descriptor();

    assert_eq!(expected, actual);
}

#[test_case]
fn test_SegmentDescriptor_kernel_mode_code_segment() {
    let expected = SegmentDescriptor {
        limit_low: 0xFFFF,
        base_low: 0,
        base_mid: 0,
        access_byte: 0x9A,
        limit_flags: 0xAF,
        base_high: 0,
    };

    let actual = SegmentDescriptor:: kernel_mode_code_segment();

    assert_eq!(expected, actual);
}

#[test_case]
fn test_SegmentDescriptor_kernel_mode_data_segment() {
    let expected = SegmentDescriptor {
        limit_low: 0xFFFF,
        base_low: 0,
        base_mid: 0,
        access_byte: 0x92,
        limit_flags: 0xCF,
        base_high: 0,
    };

    let actual = SegmentDescriptor:: kernel_mode_data_segment();

    assert_eq!(expected, actual);
}

#[test_case]
fn test_SegmentDescriptor_user_mode_code_segment() {
    let expected = SegmentDescriptor {
        limit_low: 0xFFFF,
        base_low: 0,
        base_mid: 0,
        access_byte: 0xFA,
        limit_flags: 0xAF,
        base_high: 0,
    };

    let actual = SegmentDescriptor:: user_mode_code_segment();

    assert_eq!(expected, actual);
}

#[test_case]
fn test_SegmentDescriptor_user_mode_data_segment() {
    let expected = SegmentDescriptor {
        limit_low: 0xFFFF,
        base_low: 0,
        base_mid: 0,
        access_byte: 0xF2,
        limit_flags: 0xCF,
        base_high: 0,
    };

    let actual = SegmentDescriptor:: user_mode_data_segment();

    assert_eq!(expected, actual);
}

#[test_case]
fn test_SystemSegmentDescriptor_tss() {
    todo!();
}