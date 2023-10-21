use limine::{
    BootTimeRequest, 
    HhdmRequest, 
    MemmapRequest, 
    StackSizeRequest, 
    SmpRequest,
};
use x86_64::instructions::interrupts;

use crate::logging;

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
    interrupts::disable();
    logging::init();
    let cpu_count = SMP.get_response().get().expect("Error getting SMP response from Limine").cpu_count;
    log::info!("cpu count: {}", cpu_count);
}