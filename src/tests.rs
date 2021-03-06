use alloc::{
    boxed::Box,
    vec::Vec
};

use crate::{serial_println, serial_print, println, print};
use tinix::qemu::{QemuExitCode, exit_qemu};
use tinix::gfx::vga::{Color};
use tinix::gfx;
use tinix_alloc::allocator;
use tinix::gfx::drawables::Drawable;


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


#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let window = tinix::gfx::windows::panic_window::from(info);
    window.draw_self();
    tinix::gfx::swap();
    serial_println!("{}",info);
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