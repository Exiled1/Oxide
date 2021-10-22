// I really want the ability to backtrace on a panic without the complex methods that rust uses. So we're reworking one of the labs from JOS.
use crate::println;
// use crate::serial_println;
// use crate::serial_print;

pub fn backtrace(){
    
    println!("Stack trace:");
    // First make something to read the base pointer.
    // for our system this is a 64 bit wide pointer. 
    // However, this should still work if built for a 32-bit target.
    let mut base_pointer: *const usize; 

    unsafe{ asm!("mov {}, rbp", out(reg) base_pointer) }; // Hopefully assigns the base pointer to us haha.

    while !base_pointer.is_null(){ // Walk the stack.
        let caller = unsafe{ *base_pointer.offset(1) } as usize; // 1 sized offset from the base is the caller.
        println!("  Caller: {:#x}", caller);
        base_pointer = unsafe{ (*base_pointer) as *const usize };
    }
    
    // As a fun future challenge, maybe add a symbol table and make a parser for the symbols too.
    // TODO: When virtual memory is implemented, make the base pointer map to virtual memory.
}