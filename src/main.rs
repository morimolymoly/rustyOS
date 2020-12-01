#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rusty_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
extern crate rlibc;
mod vga_buffer;
mod serial;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rusty_os::init(); // init routine

    /*
    unsafe {
        *(0xdeafbeef as *mut u64) = 1000;
    };
    x86_64::instructions::interrupts::int3();
    */

    #[cfg(test)]
    test_main();

    rusty_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    rusty_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    rusty_os::test_panic_handler(_info);
    rusty_os::hlt_loop();
}
