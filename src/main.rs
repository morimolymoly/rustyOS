#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rusty_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
extern crate rlibc;
mod vga_buffer;
mod serial;

use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");
    println!("bootinfo {:#?}", boot_info);

    rusty_os::init(); // init routine

    use x86_64::registers::control::Cr3;
    let (level4_page_table, _) = Cr3::read();
    println!("level4 page table is here {:?}", level4_page_table.start_address());

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
