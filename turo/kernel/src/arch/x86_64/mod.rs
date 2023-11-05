
pub mod io;

/// Architecture-specific initialization function
/// 
/// Initializes the kernel on the x86_64 architecture 
pub fn arch_main() {
    unsafe {
        asm!("cli");
        crate::drivers::uart_16550::init();
        crate::util::logger::init();
        log::trace!("logging works!");
        asm!("sti");
    }
}