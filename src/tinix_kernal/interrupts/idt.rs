use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::structures::idt::{
    InterruptDescriptorTable,
    InterruptStackFrame,
};
use crate::println;

lazy_static! {
    static ref IDT : InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint);
        idt.double_fault.set_handler_fn(double_fault);

        idt
    };
}

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn double_fault(_info : &mut InterruptStackFrame, _ec : u64) -> ! {
    panic!("Double Fault [{}]:\n{:#?}",_ec,_info);
}

extern "x86-interrupt" fn breakpoint(_info : &mut InterruptStackFrame) {
    println!("Breakpoint:\n{:#?}",_info);
}