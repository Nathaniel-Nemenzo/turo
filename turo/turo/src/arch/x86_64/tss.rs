
#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct TaskStateSegment {
    _0:         u32,
    rsp:        [u64; 3],
    _1:         u32,
    _2:         u32,
    ist:        [u64; 7],
    _3:         u32,
    _4:         u32,
    _5:         u16,
    iobp:       u16,
}   