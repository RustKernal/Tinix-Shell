//Copyright 2021, George Venn, GPL v3.0, NO WARRANTY

//We can't use the Rust Standard Library as a Baremetal Application
#![no_std]
#![feature(decl_macro)]
#![feature(abi_x86_interrupt)]


pub mod io;
pub mod qemu;
pub mod gfx;
pub mod interrupts;


pub fn init_modules() {
    interrupts::init();
}