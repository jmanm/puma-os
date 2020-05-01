#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(puma_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use puma_os::{serial_print, serial_println};

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use puma_os::allocator;
    use puma_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    puma_os::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    puma_os::test_panic_handler(info)
}

#[test_case]
fn simple_allocation() {
    serial_print!("simple_allocation... ");
    let heap_val = Box::new(22);
    assert_eq!(*heap_val, 22);
    serial_println!("[ok]");
}

#[test_case]
fn large_vec() {
    serial_print!("large_vec... ");
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
    serial_println!("[ok]");
}

use puma_os::allocator::HEAP_SIZE;

#[test_case]
fn many_boxes() {
    serial_print!("many_boxes... ");
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    serial_println!("[ok]");
}