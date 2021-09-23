#![feature(abi_x86_interrupt)]

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;

pub fn initalize_idt(){
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame){
    println!("EXECPTION: BREAKPOINT\n{:#?}", stack_frame);
}