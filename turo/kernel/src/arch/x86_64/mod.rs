pub mod gdt;
pub mod tss;
pub mod serial;

/// Architecture-specific initialization function
/// 
/// Initializes the kernel on the x86_64 architecture 
pub fn arch_main() {
    gdt::init();
}