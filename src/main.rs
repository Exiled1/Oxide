#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(crate::my_test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_wrapper;

use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

pub(crate) const QEMU_EXIT_PORT: i32 = 0xf4;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World {} asdfasfasdfasfsafasdfasdfadsfsadfasdfsadfasdfasdfadfasdfasdfadsfasfsadfadsfasdfasfsadfasdfadsfadsfadsfadsfadsfadsfadsfadsfasdfadsfadsfadsfasdfasdfadsf", "!");
    println!(
        "This should write to the next line {}, {}",
        "does it?", b'b'
    );

    #[cfg(test)]
    test_main();

    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCodes {
    Success = 0x10,
    Failure = 0x11,
}

pub fn qemu_exit(exit_code: QemuExitCodes) {
    let mut port = Port::new(QEMU_EXIT_PORT as u16);
    unsafe {
        port.write(exit_code as u32);
    }
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// -------------- Testing Suite below -------------
pub trait Testable {
    fn run(&self) -> ();
}
impl <T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}
#[cfg(test)]
fn my_test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running internal tests, total tests: {}", tests.len());
    for test in tests {
        test.run();
    }
    qemu_exit(QemuExitCodes::Success);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}


// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    qemu_exit(QemuExitCodes::Failure);
    loop {}
}