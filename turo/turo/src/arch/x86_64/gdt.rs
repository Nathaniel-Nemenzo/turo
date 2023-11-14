use bitflags::bitflags;

use super::tss::TaskStateSegment;

pub fn init() {}

#[derive(Debug, Clone)]
pub struct GlobalDescriptorTable {
    table: [u64; 8],
    len:   u64,    
}

impl GlobalDescriptorTable {
    /// Creates a new, empty GDT
    /// 
    /// Provides some default functionality (**subject to change as the kernel
    /// evolves**):
    /// - Entry 0 is the null descriptor
    /// - Entry 1 is the kernel mode code segment
    /// - Entry 2 is the kernel mode data segment
    /// - Entry 3 is the user mode code segment
    /// - Entry 4 is the user mode data segment
    /// - Entry 5 is the task state segment (64-bit system segment)
    pub fn new() {

    }

    pub fn add_entry() {

    }

    pub fn load() {}
}

#[derive(Debug, Clone)]
pub enum SegmentDescriptor {
    Default(DefaultSegmentDescriptor),
    System(SystemSegmentDescriptor),
}

bitflags! {
    /// A struct representing the flags on a `SegmentDescriptor`. The `System` variant of the
    /// `SegmentDescriptor` has a different access byte format, so bits 40-47 are invalidated.
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct DefaultSegmentDescriptorFlags: u64 {
        /// Accessed bit. The CPU will set it when the segment is accessed unless set to 1 in
        /// advance. In case the GDT descriptor is stored in read-only pages and this bit
        /// is set to 0, the CPU will trigger a page fault. Best to set to 1 unless otherwise
        /// needed.
        const ACCESSED = 1 << 40;

        /// Readable/writable bit. Readable bit for code segments. If 0, read access for this
        /// segment is not allowed. If 1, read access is allowed. Write access never allowed for
        /// code segments. Writable bit for data segments. If 0, write access is not allowed, if
        /// 1, allowed. Read access is always allowed for data segments.
        const READABLE_WRITABLE = 1 << 41;

        /// Direction / conforming bit. For data selectors: direction bit. If 0, segment grows up, 
        /// if 1, segment grows down. For code selectors: conforming bit. If 0, code in segment
        /// can only be executed from ring set in `DPL`. If 1, code can be executed by equal
        /// or lower privilege level.
        const DIRECTION_CONFORMING = 1 << 42;

        /// Executable bit. If 0, descriptor defines a data segment. If 1, defines a code segment.
        const EXECUTABLE = 1 << 43;

        /// Descriptor-type bit. If 0, system segment; if 1 code or data segment.
        const CODE_DATA_SEGMENT = 1 << 44;

        /// DPL level field(s). Contains the CPU privilege level of the segment. 0 == kernel, 3 == user
        const DPL_RING_0 = 0 << 45;
        // const DPL_RING_1 = 1 << 45;
        // const DPL_RING_2 = 2 << 45;
        const DPL_RING_3 = 3 << 45;

        /// Present bit. Allows an entry to refer to a valid segment. Must be set to 1 for any valid segment.
        const PRESENT = 1 << 47; 

        const RESERVED = 1 << 52;

        /// Long-mode code flag. If 1, descriptor defines a 64-bit code segment. When set, DB 
        /// should always be clear. For any other type of segment, it should be clear
        const LONG_MODE = 1 << 53;

        /// Size flag. If 0, descriptor defines a 16-bit protected codes egment. If 1, defines a 32
        /// bit protected code segment.
        const PROTECTED_MODE_32 = 1 << 54;

        /// Granularity flag. If 0, limit is in 1-byte blocks. If 1, limit is in 4KiB blocks
        const PAGE_BLOCKS = 1 << 55;
    }
}

bitflags! {
    pub struct SystemSegmentDescriptorTypeFlags: u128 {
        /// The type of the system segment.
        /// Types available in 32-bit protected mode:
        /// 0x1: 16-bit TSS (Available)
        /// 0x2: LDT
        /// 0x3: 16-bit TSS (Busy)
        /// 0x9: 32-bit TSS (Available)
        /// 0xB: 32-bit TSS (Busy)
        /// Types available in Long Mode:
        const PROTECTED_MODE_16_BIT_TSS_AVAILABLE = 1 << 40;
        const PROTECTED_MODE_LDT = 2 << 40;
        const PROTECTED_MODE_16_BIT_TSS_BUSY = 3 << 40;
        const PROTECTED_MODE_32_BIT_TSS_AVAILABLE = 9 << 40;
        const PROTECTED_MODE_32_BIT_TSS_BUSY = 0xB << 40;

        /// 0x2: LDT
        /// 0x9: 64-bit TSS (Available)
        /// 0xB: 64-bit TSS (Busy)
        const LONG_MODE_LDT = 2 << 40;
        const LONG_MODE_64_BIT_TSS_AVAILABLE = 9 << 40;
        const LONG_MODE_64_BIT_TSS_BUSY = 0xB << 40;
    }
}

/// A segment descriptor is an entry in the GDT. There are two types of segment descriptors.
/// One type is the `default` type (user/code segments) and the other is the `system` type,
/// which is mainly for the TSS in this case.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct DefaultSegmentDescriptor {
    value:  u64,
}

impl DefaultSegmentDescriptor {

    ///
    /// null()
    /// 
    /// Creates a null DefaultSegmentDescriptor
    pub fn null() -> Self {
        Self {
            value: 0
        }
    }

    ///
    /// kernel_data()
    /// 
    /// Creates a 64-bit kernel data segment
    pub fn kernel_data() -> Self {
        use DefaultSegmentDescriptorFlags as Flags;
        
        let mut ret = Self { value: 0 };

        // First, let's set the limit.
        ret.value |= 0xFFFF;
        ret.value |= 0xF << 48;

        // Secondly, let's set the access byte (10010010 or 0x92)
        let access = Flags::from_bits_truncate(
            Flags::PRESENT.bits()
            | Flags::CODE_DATA_SEGMENT.bits()
            | Flags::READABLE_WRITABLE.bits()
        );
        ret.value |= access.bits();

        // Finally, let's set the flags (1100 or 0xC)
        let flags = Flags::from_bits_truncate(
            Flags::PAGE_BLOCKS.bits()
            | Flags::PROTECTED_MODE_32.bits()
        );
        ret.value |= flags.bits();

        ret
    }

    ///
    /// kernel_data()
    /// 
    /// Creates a 64-bit kernel code segment
    pub fn kernel_code() -> Self {
        use DefaultSegmentDescriptorFlags as Flags;
        
        let mut ret = Self { value: 0 };

        // First, let's set the limit.
        ret.value |= 0xFFFF;
        ret.value |= 0xF << 48;

        // Secondly, let's set the access byte (10011010 or 0x9A)
        let access = Flags::from_bits_truncate(
            Flags::PRESENT.bits()
            | Flags::CODE_DATA_SEGMENT.bits()
            | Flags::EXECUTABLE.bits()
            | Flags::READABLE_WRITABLE.bits()
        );
        ret.value |= access.bits();

        // Finally, let's set the flags (1010 or 0xA)
        let flags = Flags::from_bits_truncate(
            Flags::PAGE_BLOCKS.bits()
            | Flags::LONG_MODE.bits()
        );
        ret.value |= flags.bits();

        ret
    }

    ///
    /// user_data()
    /// 
    /// Creates a 64-bit user data segment
    pub fn user_data() -> Self {
        use DefaultSegmentDescriptorFlags as Flags;
        
        let mut ret = Self { value: 0 };

        // First, let's set the limit.
        ret.value |= 0xFFFF;
        ret.value |= 0xF << 48;

        // Secondly, let's set the access byte (11110010 or 0xF2)
        let access = Flags::from_bits_truncate(
            Flags::PRESENT.bits()
            | Flags::DPL_RING_3.bits()
            | Flags::CODE_DATA_SEGMENT.bits()
            | Flags::READABLE_WRITABLE.bits()
        );
        ret.value |= access.bits();

        // Finally, let's set the flags (1100 or 0xC)
        let flags = Flags::from_bits_truncate(
            Flags::PAGE_BLOCKS.bits()
            | Flags::PROTECTED_MODE_32.bits()
        );
        ret.value |= flags.bits();

        ret
    }

    ///
    /// user_code()
    /// 
    /// Creates a 64-bit user code segment
    pub fn user_code() -> Self {
        use DefaultSegmentDescriptorFlags as Flags;
        
        let mut ret = Self { value: 0 };

        // First, let's set the limit.
        ret.value |= 0xFFFF;
        ret.value |= 0xF << 48;

        // Secondly, let's set the access byte (11111010 or 0xFA)
        let access = Flags::from_bits_truncate(
            Flags::PRESENT.bits()
            | Flags::DPL_RING_3.bits()
            | Flags::CODE_DATA_SEGMENT.bits()
            | Flags::EXECUTABLE.bits()
            | Flags::READABLE_WRITABLE.bits()
        );
        ret.value |= access.bits();

        // Finally, let's set the flags (1010 or 0xA)
        let flags = Flags::from_bits_truncate(
            Flags::PAGE_BLOCKS.bits()
            | Flags::LONG_MODE.bits()
        );
        ret.value |= flags.bits();

        ret
    }
}

/// A system segment descriptor alters the format of the access byte in
/// `DefaultSegmentDescriptor` slightly and has a size of 128 bits in long-mode,
/// which is used to ensure that the base value can contain a 64-bit linear address.
/// 
/// The system segment is mainly used for the TSS.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct SystemSegmentDescriptor {
    value:  u128,
}

impl SystemSegmentDescriptor {
    ///
    /// from_tss(&TaskStateSegment)
    /// 
    /// Creates a system segment descriptor from the provided TSS
    pub fn from_tss(tss: &TaskStateSegment) {

    }
}