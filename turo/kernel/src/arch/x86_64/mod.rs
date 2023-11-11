
pub mod io;
// pub mod gdt;
pub mod serial;

/// Architecture-specific initialization function
/// 
/// Initializes the kernel on the x86_64 architecture 
pub fn arch_main() {
    unsafe {
        x86_64::instructions::interrupts::disable();
            crate::util::logger::init();
            log::trace!("printing to host");
        x86_64::instructions::interrupts::enable();
    }
}