//Copyright 2021, George Venn, GPL v3.0, NO WARRANTY

//We can't use the Rust Standard Library as a Baremetal Application
#![no_std]
//We also can't use the normal Main entry point
#![no_main] 
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(dead_code)]

extern crate alloc;

mod shell;
mod tests;
mod tvm;

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
use tinix::gfx::vga::Pixel;



//use tinix::tasks::{Task, executor::Executor, keyboard};

use bootloader::BootInfo;
use bootloader::entry_point;
use x86_64::VirtAddr;
use x86_64::structures::paging::Translate;
use x86_64::structures::paging::Page;


entry_point!(shell_main);



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
    //gfx::set_gfx_mode(tinix::gfx::vga::VgaMode::TEXT_80x25);
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
    

    //let mut exec = Executor::new();
    //exec.spawn(Task::new(shell::shell_task()));
    gfx::clear(Color::Blue);
    //exec.run();

    tinix::set_tick_rate(1000);
    
    shell::shell_task();

    loop {tinix::pause(1)}
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}



pub fn get_used() -> usize {
    ALLOCATOR.lock().used()
}

pub fn get_free() -> usize {
    ALLOCATOR.lock().free()
}

pub fn get_size() -> usize {
    ALLOCATOR.lock().size()
}

pub fn get_available() -> usize {
    let alloc = ALLOCATOR.lock();
    alloc.size() - alloc.used() 
}