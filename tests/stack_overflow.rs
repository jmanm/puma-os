#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use puma_os::{exit_qemu, QemuExitCode, serial_print, serial_println};

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(puma_os::gdt::DOUBLE_FAULT_IST_INDEX);

        }

        idt
    };
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow... ");

    puma_os::gdt::init();
    init_test_idt();

    stack_overflow();

    panic!("[failed]");
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    puma_os::test_panic_handler(info)
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}