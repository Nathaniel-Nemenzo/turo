// pub mod gdt;
pub mod serial;

/// Architecture-specific initialization function
/// 
/// Initializes the kernel on the x86_64 architecture 
pub fn arch_main() {
    log::trace!("Initializing on x86_64");
    x86_64::instructions::interrupts::disable();
    // x86_64::instructions::interrupts::enable();
}