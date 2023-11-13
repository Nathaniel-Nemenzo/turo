#![feature(
    prelude_import,
    custom_test_frameworks,
)]

// Testing
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

#![allow(internal_features)]
#![no_std]
#![no_main]

mod prelude {
    pub mod rust_2021 {
        pub use core::arch::asm;
        pub use core::prelude::rust_2021::*;
    }
}

#[prelude_import]
pub use prelude::rust_2021::*;

use core::panic::PanicInfo;

pub mod arch;
pub mod util;
pub mod drivers;

pub fn init() {
    crate::util::logger::init().expect("Could not initialize logger.");
    arch::arch_main();
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where 
    T: Fn(),
{
        fn run(&self) -> () {
            serial_print!("{}...\t", core::any::type_name::<T>());
            self();
            serial_println!("[ok]");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {} 
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}