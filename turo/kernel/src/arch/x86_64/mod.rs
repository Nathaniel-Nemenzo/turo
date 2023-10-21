use limine::{
    BootTimeRequest, 
    FramebufferRequest, 
    HhdmRequest, 
    MemmapRequest, 
    StackSizeRequest, 
    SmpInfo, SmpRequest,
};
use x86::cpuid::CpuId;
use x86_64::instructions::interrupts;

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
    // Make sure we can read the kernel stack
    unsafe {
        core::ptr::read_volatile(
            STACK
                .get_response()
                .as_ptr()
                .expect("Expected kernel stack to be readable"),
        );
    }

    interrupts::disable();

    // Set the physical memory offset for virtual addressing
    crate::PHYSICAL_OFFSET.call_once(|| {
        HHDM
            .get_response()
            .get()
            .expect("Error getting HHDM response from Limine")
            .offset.try_into().unwrap()
    });

    // Check if we have features
    let features = CpuId::new()
        .get_feature_info()
        .expect("Error getting CpuId info");
    assert!(features.has_xsave(), "XSAVE not available");
    assert!(features.has_mmx(), "MMX not available");
    assert!(features.has_fpu(), "FPU not available");
    assert!(features.has_sse(), "SSE not available");
}