#![no_std]
#![no_main]
mod vga_wrapper;

use core::panic::PanicInfo;

//static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    //vga_wrapper::print_test("I have a working color VGA wrapper! Hello World!");
    //vga_wrapper::print_test("This is going to crash");
    println!("Hello World {} asdfasfasdfasfsafasdfasdfadsfsadfasdfsadfasdfasdfadfasdfasdfadsfasfsadfadsfasdfasfsadfasdfadsfadsfadsfadsfadsfadsfadsfadsfasdfadsfadsfadsfasdfasdfadsf", "!");
    println!("This should write to the next line {}", "does it?");
    panic!("Test panic message! Grrrr");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}