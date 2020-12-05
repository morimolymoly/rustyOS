#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rusty_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
extern crate rlibc;
mod vga_buffer;
mod serial;

extern crate alloc;
use alloc::boxed::Box;

use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rusty_os::memory;
    use x86_64::{structures::paging::{Page, Size4KiB}, VirtAddr};
    use rusty_os::allocator;

    println!("Hello World{}", "!");
    rusty_os::init(); // init routine

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe{ memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    let page: Page<Size4KiB> = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap init failed!");

    let a = Box::new(10);


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
