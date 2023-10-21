use limine::{
    BootTimeRequest, 
    HhdmRequest, 
    MemmapRequest, 
    StackSizeRequest, 
    SmpRequest,
};

// Limine Requests
static HHDM: HhdmRequest = HhdmRequest::new(0);
static STACK: StackSizeRequest = StackSizeRequest::new(0).stack_size(4096 * 32 as u64);
static BOOT_TIME: BootTimeRequest = BootTimeRequest::new(0);
static MEM_MAP: MemmapRequest = MemmapRequest::new(0);
static SMP: SmpRequest = SmpRequest::new(0);

/// Architecture-specific initialization function
/// 
/// Initializes the kernel on the x86_64 architecture 
pub fn arch_main() {
}