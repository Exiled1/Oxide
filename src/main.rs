#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(oxide::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(asm)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use oxide::println;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point}; 
// Bringing these in to quickly redefine my entry point and because the crate I'm using passes 
// the memory map/physical memory abstraction which is passed over during the boot process
use oxide::kdebug::backtrace;

entry_point!(kmain); // Use this to make it so the bootloader crate loads our kmain.

#[no_mangle]
fn kmain(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");
    
    #[cfg(test)]
    test_main();
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    // Print a stack trace 
    backtrace();
    loop {};
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    backtrace();
    oxide::test_panic_handler(info)
}

// ---------- Tests ----------

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}