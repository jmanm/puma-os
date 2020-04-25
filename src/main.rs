#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(puma_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use puma_os::println;

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

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("H3ll0 from Puma OS");

    puma_os::init();

    #[cfg(test)]
    test_main();

    puma_os::hlt_loop();
}
