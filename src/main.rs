#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

use core::panic::PanicInfo;
use puma_os::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    puma_os::hlt_loop();
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use puma_os::interrupts::PICS;
    let maj_ver = 0;
    let min_ver = 1;
    let patch_ver = 0;

    println!("Welcome to Puma OS");
    println!("Version {}.{}.{}", maj_ver, min_ver, patch_ver);

    puma_os::gdt::init();
    puma_os::interrupts::init_idt();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    puma_os::hlt_loop();
}
