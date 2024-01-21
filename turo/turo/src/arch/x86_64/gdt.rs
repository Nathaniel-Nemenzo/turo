use bitflags::bitflags;
use lazy_static::lazy_static;
use x86_64::{registers::segmentation::Segment, structures::idt::DescriptorTable};
use core::mem::size_of;

use super::tss::{TaskStateSegment, TSS};

/// Define bits for access flag for SegmentDescriptor.
bitflags! {
    /// DescriptorFlags are shared among all descriptor types
    pub struct DescriptorFlags: u8 {
        const LONG_MODE_CODE_SEGMENT = 1 << 1;
        const PROTECTED_MODE_SEGMENT = 1 << 2;
        const PAGE_BLOCKS = 1 << 3;
    } 

    /// SegmentDescriptorFlags defines and implements all of the flags used in a SegmentDescriptor.
    pub struct SegmentDescriptorAccessByte: u8 {
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
                SegmentDescriptorAccessByte::PRESENT.bits() | 
                SegmentDescriptorAccessByte::DPL_0.bits() |
                SegmentDescriptorAccessByte::CODE_DATA_SEGMENT.bits() |
                SegmentDescriptorAccessByte::EXECUTABLE.bits() |
                SegmentDescriptorAccessByte::CODE_SEGMENT_READABLE.bits()
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
                SegmentDescriptorAccessByte::PRESENT.bits() | 
                SegmentDescriptorAccessByte::DPL_0.bits() |
                SegmentDescriptorAccessByte::CODE_DATA_SEGMENT.bits() |
                SegmentDescriptorAccessByte::DATA_SEGMENT_WRITABLE.bits()
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
                SegmentDescriptorAccessByte::PRESENT.bits() | 
                SegmentDescriptorAccessByte::DPL_3.bits() |
                SegmentDescriptorAccessByte::CODE_DATA_SEGMENT.bits() |
                SegmentDescriptorAccessByte::EXECUTABLE.bits() |
                SegmentDescriptorAccessByte::CODE_SEGMENT_READABLE.bits()
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
                SegmentDescriptorAccessByte::PRESENT.bits() | 
                SegmentDescriptorAccessByte::DPL_3.bits() |
                SegmentDescriptorAccessByte::CODE_DATA_SEGMENT.bits() |
                SegmentDescriptorAccessByte::DATA_SEGMENT_WRITABLE.bits()
            },
            limit_flags: {
                (DescriptorFlags::PAGE_BLOCKS.bits() | 
                DescriptorFlags::PROTECTED_MODE_SEGMENT.bits()) << 4 | 0xF
            },
            base_high: 0
        }
    }
}

impl Into<u64> for SegmentDescriptor {
    fn into(self) -> u64 {
        let mut ret = (self.base_high as u64) << 56;
        ret |= (self.limit_flags as u64) << 48; 
        ret |= (self.access_byte as u64) << 40;
        ret |= (self.base_mid as u64) << 32;
        ret |= (self.base_low as u64) << 16;
        ret |= self.limit_low as u64;
        ret
    }
}

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
    pub fn tss(tss: &TaskStateSegment) -> Self {
        let tss_size = size_of::<TaskStateSegment>();
        let tss_addr = tss as *const _ as u64;
        Self {
            limit_low: (0xFFFF & tss_size) as u16,
            base_low: (0xFFFF & tss_addr) as u16,
            base_mid_low: (0xFF & tss_addr >> 16) as u8,
            access_byte: SegmentDescriptorAccessByte::PRESENT.bits() | SystemSegmentDescriptorAccessByte::TSS_64_AVAILABLE.bits(),
            limit_flags: (0xF & tss_size >> 16) as u8,
            base_mid_high: (0xFF & tss_addr >> 24) as u8,
            base_high: (0xFFFFFFFF & tss_addr >> 32) as u32,

            // hopefully doesn't cause any issues
            _reserved: 0,
        }
    }
}

impl Into<(u64, u64)> for SystemSegmentDescriptor {
    fn into(self) -> (u64, u64) {
        let mut ret = ((self.base_mid_high as u64) << 56, (self._reserved as u64) << 32);
        
        // lower half
        ret.0 |= (self.limit_flags as u64) << 48; 
        ret.0 |= (self.access_byte as u64) << 40;
        ret.0 |= (self.base_mid_low as u64) << 32;
        ret.0 |= (self.base_low as u64) << 16;
        ret.0 |= self.limit_low as u64;

        // upper half
        ret.1 |= (self.base_high as u64);
        ret
    }
}

/// The GlobalDescriptorTable is a data structure specific to IA-32 and x86-64 architectures. It contains entries telling the CPU about memory segments. [https://wiki.osdev.org/Global_Descriptor_Table]. 
lazy_static! {
    static ref GDT: [u64; 7] = [
        SegmentDescriptor::null_descriptor().into(),
        SegmentDescriptor::kernel_mode_code_segment().into(),
        SegmentDescriptor::kernel_mode_data_segment().into(),
        SegmentDescriptor::user_mode_code_segment().into(),
        SegmentDescriptor::user_mode_data_segment().into(),
        <SystemSegmentDescriptor as core::convert::Into<(u64, u64)>>::into(SystemSegmentDescriptor::tss(&TSS)).0,
        <SystemSegmentDescriptor as core::convert::Into<(u64, u64)>>::into(SystemSegmentDescriptor::tss(&TSS)).1,
    ];
}


// unit tests ------------------------------------

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
fn test_SegmentDescriptor_into64() {
    let descriptor = SegmentDescriptor {
        limit_low: 0xABCD,
        base_low: 0xABCD,
        base_mid: 0xAB,
        access_byte: 0xAB,
        limit_flags: 0xAB,
        base_high: 0xAB,
    };

    let expected: u64 = 0xABABABABABCDABCD;
    let actual: u64 = descriptor.into();
    assert_eq!(expected, actual)
}

#[test_case]
fn test_SystemSegmentDescriptor_tss() {
    let tss = TaskStateSegment::null();
    let tss_size = size_of::<TaskStateSegment>();
    let tss_addr = &tss as *const _ as u64;
    let expected = SystemSegmentDescriptor {
        limit_low: 0xFFFF & tss_size as u16,
        base_low: 0xFFFF & tss_addr as u16,
        base_mid_low: 0xFF & (tss_addr >> 16) as u8,
        access_byte: 0x89,
        limit_flags: 0xF & (tss_size >> 16) as u8,
        base_mid_high: 0xFF & (tss_addr >> 24) as u8,
        base_high: 0xFFFFFFFF & (tss_addr >> 32) as u32,
        _reserved: 0
    };
}

#[test_case]
fn test_SystemSegmentDescriptor_tss() {
    let descriptor = SystemSegmentDescriptor {
        limit_low: 0xABCD,
        base_low: 0xABCD,
        base_mid_low: 0xAB,
        access_byte: 0xAB,
        limit_flags: 0xAB,
        base_mid_high: 0xAB,
        base_high: 0xABCD,
        _reserved: 0
    };

    let expected: (u64, u64) = (0xABABABABABCDABCD, 0xABCD);
    let actual: (u64, u64) = descriptor.into();
}