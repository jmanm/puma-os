#![no_std]
#![no_main]

use core::panic::PanicInfo;
use puma_os::{QemuExitCode, exit_qemu, serial_print, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[failed]");
    exit_qemu(QemuExitCode::Failed);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn should_fail() {
    serial_print!("should_fail... ");
    assert_eq!(1, 0);
}