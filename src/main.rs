//Copyright 2021, George Venn, GPL v3.0, NO WARRANTY

//We can't use the Rust Standard Library as a Baremetal Application
#![no_std]
//We also can't use the normal Main entry point
#![no_main] 
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

//Imports
use core::panic::PanicInfo;
use tinix_kernal::qemu::{QemuExitCode, exit_qemu};

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

pub fn test_runner(_tests: &[&dyn Fn()]) {
    exit_qemu(QemuExitCode::Success);
}
