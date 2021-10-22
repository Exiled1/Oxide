#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(oxide::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(asm)]
#![allow(unused_imports)]

use oxide::println;
use core::panic::PanicInfo;
use oxide::kdebug::backtrace;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    rawr();
    #[cfg(test)]
    test_main();
    loop {}
}
fn rawr() -> (){
    rawr2()
}

fn rawr2() -> (){
    rawr3()
}
fn rawr3() -> (){
    rawr4()
}
fn rawr4() -> (){
    rawr5()
}
fn rawr5() -> (){
    panic!();
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