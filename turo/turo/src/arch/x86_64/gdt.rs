use x86_64::instructions::interrupts;

#[repr(transparent)]
pub struct SegmentDescriptorFlags(u8);



#[repr(transparent)]
pub struct SegmentDescriptorAccess(u8);


pub fn init() {
    // Before continuing, make sure interrupts are disabled
    assert!(!interrupts::are_enabled());
}

struct GDTDescriptor {
    size:           u16,
    offset:         u64,
}

#[repr(C, packed)]
struct SegmentDescriptor {
    limit:          u16,
    base_low:       u16,
    base_medium:    u8,
    access:         u8,

    // The lower four bits correspond to the `limit` field and the upper four bits correspond to the `flags` field.
    granularity:    u8,

    base_high:      u8,
}

impl SegmentDescriptor {
    ///
    /// new()
    /// 
    /// Create a new GlobalDescriptorEntry
    /// 
    /// User doesn't have to set the `base` and `limit` values in long mode, they are ignored as the descriptor will
    /// cover the entire linear address space regardless of what they are set to.
    pub fn new(access: SegmentDescriptorAccess, flags: SegmentDescriptorFlags) -> SegmentDescriptor {
        SegmentDescriptor {
            limit:       0,
            base_low:    0,
            base_medium: 0,
            access:      access.bits(),
            granularity: flags.bits() & 0xF0,
            base_high:   0,
        }
    }
}