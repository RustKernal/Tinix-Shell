//Copyright 2021, George Venn, GPL v3.0, NO WARRANTY

//We can't use the Rust Standard Library as a Baremetal Application
#![no_std]
//We also can't use the normal Main entry point
#![no_main] 
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
//Imports
use tinix::qemu::{
    QemuExitCode, exit_qemu
};
use tinix::{
    serial_print, serial_println, print, println
};

use tinix::gfx::vga::{
    ColorCode, Char, Color
};

use tinix::gfx;

use tinix::interrupts::pit::set_frequency;
use tinix::allocation::memory::active_level_4_table;

use core::panic::PanicInfo;

use bootloader::BootInfo;
use bootloader::entry_point;
use x86_64::VirtAddr;

entry_point!(shell_main);

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
pub fn shell_main(boot_info : &BootInfo)-> ! {
    test_main();
    loop {}
}

/// Entry point for `cargo run`
#[cfg(not(test))]
#[no_mangle]
pub fn shell_main(boot_info : &BootInfo) -> ! {
    tinix::init_modules(boot_info);
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }

    loop {tinix::pause(1)}
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
    assert_eq!(Char::new(b'T',ColorCode::from_colors(Color::White, Color::Blue)), tinix::io::terminal::get_char(0,23));
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");

    }
}

#[test_case]
fn test_init_kernal() {
    tinix::init_modules_no_alloc();
}

#[test_case]
fn test_breakpoints() {
    x86_64::instructions::interrupts::int3(); // new;
}

#[test_case]
fn test_set_cell() {
    gfx::set_cell_color(0, 0, Color::Black, Color::Cyan);
    assert_eq!(gfx::get_bg(0,0),Color::Cyan);
}

#[test_case]
fn test_clear() {
    gfx::set_cell_color(0, 0, Color::Green, Color::Brown);
    gfx::clear(Color::Blue);
    assert_eq!(gfx::get_bg(0,0),Color::Blue);
} 