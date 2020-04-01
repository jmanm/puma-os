#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![feature(asm)]

use core::panic::PanicInfo;
use bootloader::{bootinfo::BootInfo, entry_point};
use puma_os::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    puma_os::hlt_loop();
}

entry_point!(kernel_main);

#[cfg(not(test))]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use puma_os::interrupts::PICS;
    use puma_os::green_thread::{ThreadContext, STACK_SIZE};
    let maj_ver = 0;
    let min_ver = 1;
    let patch_ver = 0;

    println!("Welcome to Puma OS");
    println!("Version {}.{}.{}", maj_ver, min_ver, patch_ver);

    puma_os::gdt::init();
    puma_os::interrupts::init_idt();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    let mut recursive_page_table = unsafe {
        puma_os::memory::init(boot_info.p4_table_addr as usize)
    };
    let mut frame_allocator = puma_os::memory::init_frame_allocator(&boot_info.memory_map);
    puma_os::memory::create_mapping(&mut recursive_page_table, &mut frame_allocator);

    let mut ctx = ThreadContext::default();
    let mut stack = vec![0_u8; STACK_SIZE as usize];
    let stack_ptr = stack.as_mut_ptr();

    unsafe {
        std::ptr::write(stack_ptr.offset(STACK_SIZE - 16) as *mut u64, hello as u64);
        ctx.rsp = stack_ptr.offset(STACK_SIZE - 16) as u64;
        gt_switch(&mut ctx);
    }

    puma_os::hlt_loop();
}

fn hello() -> ! {
    println!("NEW STACK, SON");

    loop {}
}

unsafe fn gt_switch(new: *const ThreadContext) {
    asm!("
        mov 0x00($0), %rsp
        ret
    "
    :
    : "r"(new)
    :
    : "alignstack"
    );
}