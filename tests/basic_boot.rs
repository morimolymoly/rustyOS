#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(rusty_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rusty_os::println;
extern crate rlibc;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop{}
}

fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rusty_os::test_panic_handler(info);
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
