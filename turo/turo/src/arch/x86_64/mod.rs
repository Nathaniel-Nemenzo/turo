use crate::testing::exit_qemu;


// pub mod gdt;
pub mod serial;

/// Architecture-specific initialization function
/// 
/// Initializes the kernel on the x86_64 architecture 
pub fn arch_main() {
    x86_64::instructions::interrupts::disable();
        crate::util::logger::init().expect("Could not initialize logger.");
    // x86_64::instructions::interrupts::enable();
}