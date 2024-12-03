// test/should_panic.rs

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rusty_os::{QemuExitCode, exit_qemu, serial_println};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    
    loop {}
}

fn should_fail() {
    serial_println!("should panic::should fail...\t");
    assert_eq!(0, 1);
}