#![no_std]
#![no_main]

// extern crate alloc;

#[macro_use]
pub mod vga_buffer;
pub mod arch;

use core::arch::asm;
use spin::Once;
use x86_64::VirtAddr;

pub static PHYSICAL_OFFSET: Once<usize> = Once::new();

#[inline]
pub fn phys_offset() -> VirtAddr {
    let offset = VirtAddr::new((*PHYSICAL_OFFSET.wait().unwrap()).try_into().unwrap());
    offset
}

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    arch::arch_main();
    hcf();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    hcf();
}

fn hcf() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}