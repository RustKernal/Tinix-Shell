//Copyright 2021, George Venn, GPL v3.0, NO WARRANTY

//We can't use the Rust Standard Library as a Baremetal Application
#![no_std]
//We also can't use the normal Main entry point
#![no_main] 
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

//Imports
use tinix_kernal::qemu::{QemuExitCode, exit_qemu};
use tinix_kernal::{serial_print, serial_println, print, println};
use tinix_kernal::io::terminal;
use tinix_kernal::gfx::vga::{
    Color, ColorCode, Char
};

use core::panic::PanicInfo;


pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

/// Entry point for `cargo run`
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    tinix_kernal::init_modules();
    println!("Hello World...");
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}",info);
    loop {}
}

#[test_case]
pub fn print_test() {
    println!("TEST");
    assert_eq!(Char::new(b'T',ColorCode::from_colors(Color::White, Color::Blue)), terminal::get_char(0,23));
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");

    }
}

#[test_case]
fn test_init_kernal() {
    tinix_kernal::init_modules();
}