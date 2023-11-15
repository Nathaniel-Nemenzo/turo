use bitflags::bitflags;
use core::mem::size_of;

use super::tss::TaskStateSegment;

pub fn init() {}

///
/// The GDTDescriptor struct is passed into the `lgdt` assembly instruction.
/// 
/// It contains the size of the GDT and the offset of the GDT that it describes.
#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct GDTDescriptor {
    /// The size of the GDT is the actual size - 1, because the maximum value of a 16-bit number is 65536, but the max
    /// size of a GDT can be 65536 bytes.
    size: u16,

    /// The linear address of a GDT.
    offset: u64,
}

impl GDTDescriptor {
    pub fn new(table: GlobalDescriptorTable) -> Self {
        Self {
            size: table.size - 1,

            // Gets the address of the table as a u64
            offset: &table.table as *const _ as u64,
        }
    }
}

///
/// The global descriptor table is a binary data structure specific to the IA32 and x86-64 architectures.
/// It contains entries telling the CPU about memory segments, which is an old memory management scheme used
/// in older architectures, like x86.
/// 
/// In x86-64, the global descriptor table is used mainly for system segment functionality, like the TSS descriptor;
/// it is not used for segmentation, as we use paging instead. This means that the `base` and `limit` values are never
/// used and can span the whole linear address space. 
#[derive(Debug, Clone)]
pub struct GlobalDescriptorTable {
    /// Represents the actual GDT
    /// 
    /// For now, there's a fixed limit of 8 entries (there are a maximum of 65536 / 8 == 8192 entries, but
    /// this wastes space if we initialize statically and we don't really need that much for now).
    table: [u64; 8],

    /// The size of the GDT in bytes
    size:   u16,
}

impl GlobalDescriptorTable {
    /// 
    /// new()
    /// 
    /// Creates a new, empty GDT
    /// 
    /// Adds a null descriptor (0) as the first entry in the GDT.
    pub fn new() -> Self {
        Self {
            table: [0; 8],
            size: 8,
        }
    }

    ///
    /// add_default_descriptor(&mut self, DefaultSegmentDescriptor)
    /// 
    /// Adds a default descriptor to the GDT. Default descriptors take the space of one entry in the GDT
    /// 
    /// Panics if the number of entries succeeds the limit defines in the struct
    pub fn add_default_descriptor(&mut self, descriptor: DefaultSegmentDescriptor) {
        if usize::from(self.size) == self.table.len() * size_of::<u64>() {
            panic!("No more space left in the table")
        }

        // Figure out where to add the descriptor
        let idx = usize::from(self.size) / size_of::<u64>();

        // Add the descriptor in
        self.table[idx] = descriptor.0;

        // Add to the size
        self.size += size_of::<u64>() as u16;
    }

    ///
    /// add_system_descriptor(&mut self, SystemSegmentDescriptor)
    /// 
    /// Adds a system descriptor to the GDT. System descriptors take the space of two entries in the GDT
    /// 
    /// The lower half of the system descriptor precedes the higher half in the table
    pub fn add_system_descriptor(&mut self, descriptor: SystemSegmentDescriptor) {
        // This logic is a little different because you need two entry spaces to add a system descriptor
        if usize::from(self.size) >= (self.table.len() - 1) * size_of::<u64>() {
            panic!("No more space left in the table")
        }

        // Figure out where to add the descriptor
        let idx = usize::from(self.size) / size_of::<u64>();

        // Add the descriptor
        self.table[idx] = descriptor.0;
        self.table[idx + 1] = descriptor.1;

        self.size += size_of::<u64>() as u16 * 2;
    }

    ///
    /// load(&self, &GDTDescriptor)
    /// 
    /// Loads the GDT pointer to by the GDTDescriptor
    /// 
    /// Wraps the load_unsafe() function in this struct
    pub fn load(&self, gdtr: &GDTDescriptor) {
        unsafe { self.load_unsafe(gdtr) }
    }

    ///
    /// load_unsafe(&self, &GDTDescriptor)
    /// 
    /// Loads the GDT pointed to by the GDTDescriptor
    /// 
    /// This function is unsafe because you must make sure that the underlying GDT pointed to by the GDTR is valid
    /// and you must make sure that the GDTR itself is valid.
    unsafe fn load_unsafe(&self, gdtr: &GDTDescriptor) {

    }
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
#[derive(Debug, Clone)]
pub struct DefaultSegmentDescriptor(u64);

impl DefaultSegmentDescriptor {

    ///
    /// null()
    /// 
    /// Creates a null DefaultSegmentDescriptor
    pub fn null() -> Self {
        Self(0)
    }

    ///
    /// kernel_data()
    /// 
    /// Creates a 64-bit kernel data segment
    pub fn kernel_data() -> Self {
        use DefaultSegmentDescriptorFlags as Flags;
        
        let mut ret = Self(0);

        // First, let's set the limit.
        ret.0 |= 0xFFFF;
        ret.0 |= 0xF << 48;

        // Secondly, let's set the access byte (10010010 or 0x92)
        let access = Flags::from_bits_truncate(
            Flags::PRESENT.bits()
            | Flags::CODE_DATA_SEGMENT.bits()
            | Flags::READABLE_WRITABLE.bits()
        );
        ret.0 |= access.bits();

        // Finally, let's set the flags (1100 or 0xC)
        let flags = Flags::from_bits_truncate(
            Flags::PAGE_BLOCKS.bits()
            | Flags::PROTECTED_MODE_32.bits()
        );
        ret.0 |= flags.bits();

        ret
    }

    ///
    /// kernel_data()
    /// 
    /// Creates a 64-bit kernel code segment
    pub fn kernel_code() -> Self {
        use DefaultSegmentDescriptorFlags as Flags;
        
        let mut ret = Self(0);

        // First, let's set the limit.
        ret.0 |= 0xFFFF;
        ret.0 |= 0xF << 48;

        // Secondly, let's set the access byte (10011010 or 0x9A)
        let access = Flags::from_bits_truncate(
            Flags::PRESENT.bits()
            | Flags::CODE_DATA_SEGMENT.bits()
            | Flags::EXECUTABLE.bits()
            | Flags::READABLE_WRITABLE.bits()
        );
        ret.0 |= access.bits();

        // Finally, let's set the flags (1010 or 0xA)
        let flags = Flags::from_bits_truncate(
            Flags::PAGE_BLOCKS.bits()
            | Flags::LONG_MODE.bits()
        );
        ret.0 |= flags.bits();

        ret
    }

    ///
    /// user_data()
    /// 
    /// Creates a 64-bit user data segment
    pub fn user_data() -> Self {
        use DefaultSegmentDescriptorFlags as Flags;
        
        let mut ret = Self(0);

        // First, let's set the limit.
        ret.0 |= 0xFFFF;
        ret.0 |= 0xF << 48;

        // Secondly, let's set the access byte (11110010 or 0xF2)
        let access = Flags::from_bits_truncate(
            Flags::PRESENT.bits()
            | Flags::DPL_RING_3.bits()
            | Flags::CODE_DATA_SEGMENT.bits()
            | Flags::READABLE_WRITABLE.bits()
        );
        ret.0 |= access.bits();

        // Finally, let's set the flags (1100 or 0xC)
        let flags = Flags::from_bits_truncate(
            Flags::PAGE_BLOCKS.bits()
            | Flags::PROTECTED_MODE_32.bits()
        );
        ret.0 |= flags.bits();

        ret
    }

    ///
    /// user_code()
    /// 
    /// Creates a 64-bit user code segment
    pub fn user_code() -> Self {
        use DefaultSegmentDescriptorFlags as Flags;
        
        let mut ret = Self(0);

        // First, let's set the limit.
        ret.0 |= 0xFFFF;
        ret.0 |= 0xF << 48;

        // Secondly, let's set the access byte (11111010 or 0xFA)
        let access = Flags::from_bits_truncate(
            Flags::PRESENT.bits()
            | Flags::DPL_RING_3.bits()
            | Flags::CODE_DATA_SEGMENT.bits()
            | Flags::EXECUTABLE.bits()
            | Flags::READABLE_WRITABLE.bits()
        );
        ret.0 |= access.bits();

        // Finally, let's set the flags (1010 or 0xA)
        let flags = Flags::from_bits_truncate(
            Flags::PAGE_BLOCKS.bits()
            | Flags::LONG_MODE.bits()
        );
        ret.0 |= flags.bits();

        ret
    }
}

/// A system segment descriptor alters the format of the access byte in
/// `DefaultSegmentDescriptor` slightly and has a size of 128 bits in long-mode,
/// which is used to ensure that the base value can contain a 64-bit linear address.
/// 
/// The system segment is mainly used for the TSS.
/// 
/// This is a (u64, u64) because the GDT only holds u64-sized entries. Makes the logic a bit
/// easier than having a (u128).
#[derive(Debug, Clone)]
pub struct SystemSegmentDescriptor(u64, u64);

impl SystemSegmentDescriptor {
    ///
    /// from_tss(&TaskStateSegment)
    /// 
    /// Creates a system segment descriptor from the provided TSS
    pub fn from_tss(tss: &TaskStateSegment) {

    }
}

#[test_case]
fn test_default_segment_descriptor_null() {
    let null = DefaultSegmentDescriptor::null();
    assert_eq!(null.0, 0)
}

#[test_case]
fn test_default_segment_descriptor_kernel_data() {
    let kernel_data = DefaultSegmentDescriptor::kernel_data();
    assert_eq!(kernel_data.0, 0x00_CF_92_00_0000_FFFF)
}

#[test_case]
fn test_default_segment_descriptor_kernel_code() {
    let kernel_code = DefaultSegmentDescriptor::kernel_code();
    assert_eq!(kernel_code.0, 0x00_AF_9A_00_0000_FFFF)
}

#[test_case]
fn test_default_segment_descriptor_user_data() {
    let user_data: DefaultSegmentDescriptor = DefaultSegmentDescriptor::user_data();
    assert_eq!(user_data.0, 0x00_CF_F2_00_0000_FFFF)
}

#[test_case]
fn test_default_segment_descriptor_user_code() {
    let user_code = DefaultSegmentDescriptor::user_code();
    assert_eq!(user_code.0, 0x00_AF_FA_00_0000_FFFF)
}
