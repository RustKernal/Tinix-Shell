//Copyright 2021, George Venn, GPL v3.0, NO WARRANTY

//We can't use the Rust Standard Library as a Baremetal Application
#![no_std]
//We also can't use the normal Main entry point
#![no_main] 
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

mod shell;

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

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

use tinix_alloc::memory;
use tinix_alloc::paging;
use tinix_alloc::allocator;
use core::panic::PanicInfo;

use tinix::tasks::{Task, executor::Executor, keyboard};

use bootloader::BootInfo;
use bootloader::entry_point;
use x86_64::VirtAddr;
use x86_64::structures::paging::Translate;
use x86_64::structures::paging::Page;

use alloc::{
    boxed::Box,
    vec::Vec
};

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
pub fn shell_main(boot_info : &'static BootInfo)-> ! {
    tinix::init_modules(boot_info);
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { paging::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

        unsafe {
            ALLOCATOR.lock().init(
                allocator::HEAP_START,
                allocator::HEAP_SIZE
            );
        }
    test_main();
    loop {}
}

/// Entry point for `cargo run`
#[cfg(not(test))]
#[no_mangle]
pub fn shell_main(boot_info : &'static BootInfo) -> ! {
    tinix::init_modules(boot_info);
    gfx::clear(Color::Blue);
    let mut mapper = unsafe { paging::init(VirtAddr::new(boot_info.physical_memory_offset)) };
    let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap Init failed...");

    unsafe {
        ALLOCATOR.lock().init(
            allocator::HEAP_START,
            allocator::HEAP_SIZE
        );
    }
    let default_color = (Color::White, Color::Blue);
    gfx::clear(Color::Blue);
    gfx::draw_string(0,0,"==== TINIX V0.1.0a ====",default_color);
    gfx::draw_string(0,1,"Screen Resolution: \x0280\x03x\x0225...",default_color);
    gfx::draw_string(0,2,"Heap Size: \x0e100KiB...",default_color);
    gfx::draw_string(0,3,"=======================",default_color);

    gfx::draw_rect(0,4,10,1, Color::Green);
    gfx::draw_string(11,4, " | 100% ", default_color);

    let mut exec = Executor::new();
    exec.spawn(Task::new(shell::shell_task()));
    gfx::clear(Color::Blue);
    exec.run();
    
    

    loop {tinix::pause(1)}
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
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
    tinix::disable_interrupts();
    gfx::clear(Color::Blue);
    println!("TEST");
    //assert_eq!(Char::new(b'T',ColorCode::from_colors(Color::White, Color::Blue)), tinix::io::terminal::get_char(0,23));
    tinix::enable_interrupts();
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

#[test_case]
fn simple_allocation() {
    let heap_value_1 = Box::new(41);
    let heap_value_2 = Box::new(13);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 13);
}

#[test_case]
fn large_vec() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_boxes() {
    for i in 0..allocator::HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}