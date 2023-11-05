
pub mod io;
pub mod gdt;

/// Architecture-specific initialization function
/// 
/// Initializes the kernel on the x86_64 architecture 
pub fn arch_main() {
    unsafe {
        x86_64::instructions::interrupts::disable();
            crate::drivers::uart_16550::init();
            crate::util::logger::init();
        x86_64::instructions::interrupts::enable();
    }
}