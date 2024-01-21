use lazy_static::lazy_static;

lazy_static! {
    pub static ref TSS: TaskStateSegment = TaskStateSegment::null();
}

#[repr(C, packed)]
#[derive(Debug, PartialEq)]
/// The TaskStateSegment is a binary data structure specific to the IA-32 and x86-64 architectures. It holds information about a task. In protected (32-bit) mode, the TSS is primarily suited for hardware task switching, where each individual task has its own TSS. In long (64-bit) mode, the TSS has a separate structure and is used to change the stack pointer after an interrupt or level permission change (so interrupt procedures can run on well-defined stacks).
pub struct TaskStateSegment {
    _reserved1: u32,
    rsp0: u64,
    rsp1: u64,
    rsp2: u64,
    _reserved2: u32,
    _reserved3: u32,
    ist: [u64; 7],
    _reserved4: u32,
    _reserved5: u32,
    _reserved6: u16,
    iobp: u16, 
}

impl TaskStateSegment {
    pub fn null() -> Self {
        Self {
            _reserved1: 0,
            rsp0: 0,
            rsp1: 0,
            rsp2: 0,
            _reserved2: 0,
            _reserved3: 0,
            ist: [0; 7],
            _reserved4: 0,
            _reserved5: 0,
            _reserved6: 0,
            iobp: 0
        }
    }
}