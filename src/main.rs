#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(puma_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use puma_os::println;

entry_point!(kernel_main);

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    puma_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    puma_os::test_panic_handler(info)
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("H3ll0 from Puma OS");

    puma_os::init();

    use alloc::{vec, vec::Vec, rc::Rc};
    use x86_64::VirtAddr;
    use puma_os::allocator;
    use puma_os::memory::{self, BootInfoFrameAllocator};

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    #[cfg(test)] test_main();

    puma_os::hlt_loop();
}

// fn print_page_table_contents() {
//     let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

//     println!("{:?}", phys_mem_offset);
//     for (i, entry) in l4_table.iter().enumerate() {
//         if !entry.is_unused() {
//             println!("L4 entry {}: {:?}", i, entry);

//             let phys = entry.frame().unwrap().start_address();
//             let virt = phys.as_u64() + boot_info.physical_memory_offset;
//             let ptr = VirtAddr::new(virt).as_mut_ptr();

//             use x86_64::structures::paging::PageTable;
//             let l3_table: &PageTable = unsafe { &*ptr };

//             for (i, entry) in l3_table.iter().enumerate() {
//                 if !entry.is_unused() {
//                     println!("  L3 entry {}: {:?}", i, entry);
//                 }
//             }
//         }
//     }
// }