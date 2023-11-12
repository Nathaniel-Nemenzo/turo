#![feature(
    prelude_import,
    custom_test_frameworks,
)]

// Testing
#![test_runner(crate::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![allow(internal_features)]
#![no_std]
#![no_main]

pub mod arch;
pub mod util;
pub mod drivers;
mod testing;

mod prelude {
    pub mod rust_2021 {
        pub use core::arch::asm;
        pub use core::prelude::rust_2021::*;
    }
}

#[prelude_import]
pub use prelude::rust_2021::*;

static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new(0);

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    // Ensure we got a framebuffer.
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response().get() {
        if framebuffer_response.framebuffer_count < 1 {
            hcf();
        }

        // Get the first framebuffer's information.
        let framebuffer = &framebuffer_response.framebuffers()[0];

        for i in 0..100_usize {
            // Calculate the pixel offset using the framebuffer information we obtained above.
            // We skip `i` scanlines (pitch is provided in bytes) and add `i * 4` to skip `i` pixels forward.
            let pixel_offset = i * framebuffer.pitch as usize + i * 4;

            // Write 0xFFFFFFFF to the provided pixel offset to fill it white.
            // We can safely unwrap the result of `as_ptr()` because the framebuffer address is
            // guaranteed to be provided by the bootloader.
            unsafe {
                *(framebuffer.address.as_ptr().unwrap().offset(pixel_offset as isize) as *mut u32) = 0xFFFFFFFF;
            }
        }
    }
    arch::arch_main();

    #[cfg(test)]
    test_main();

    x86_64::instructions::interrupts::enable();
    hcf();
}

#[cfg(not(test))]
#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("{}", info);
    hcf();
}

fn hcf() -> ! {
    loop {
        x86_64::instructions::interrupts::enable_and_hlt();
    }
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}