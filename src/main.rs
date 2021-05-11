//Copyright 2021, George Venn, GPL v3.0, NO WARRANTY

//We can't use the Rust Standard Library as a Baremetal Application
#![no_std]
//We also can't use the normal Main entry point
#![no_main] 

//Imports
use core::panic::PanicInfo;

//We dont want to mangle the name of this function in export
#[no_mangle]
//Export this function, using the C Calling Convention
//we also dont want this function to return...
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
pub fn panic_handler(_info : &PanicInfo) -> ! {
    loop {}
}