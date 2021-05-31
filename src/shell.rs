use core::borrow::Borrow;
use core::ops::AddAssign;

use lazy_static::lazy_static;
use tinix::gfx::widgets::TextArea;
use tinix::io::serial;
use tinix::{gfx, qemu, stdout};
use tinix::gfx::drawables::{Drawable};
use tinix::gfx::widgets::ProgressBar;
use tinix::gfx::vga::Color;
use tinix;
use tinix::interrupts::global_timer;


use spin::Mutex;

use alloc::{
    vec,
    vec::Vec,
    string::String,
    string::ToString,
    borrow::ToOwned
};

use tinix_fs;
use tinix_fs::api::{FileReader, FileWriter};

use num_format::{Buffer, Locale};

static mut FRAME_NUM : usize = 0;

static mut _V1 : Vec<Block> = Vec::new();


static mut STATE : usize = STATE_ASSEMBLE;


const STATE_COMMAND_WAIT    : usize = 1;
const STATE_ASSEMBLE        : usize = 2;

lazy_static! {
    static ref COMMAND : Mutex<String> = Mutex::new(String::new());
}

fn get_state() -> usize {
    unsafe {STATE}
}

fn set_state(x : usize) {
    unsafe {STATE = x}
}

#[allow(dead_code)]
pub fn shell_task() -> ! {
    loop {
        match get_state() {
            STATE_ASSEMBLE => state_assemble(),
            STATE_COMMAND_WAIT => state_command_wait(),
            _ => {} //Do Nothing
        }
    }
}

fn process_command(command_string : &String) {
    tinix::serial_println!("String: {:?}", command_string.as_bytes());
    match command_string.as_str() {
        
        "kill" => qemu::exit_qemu(qemu::QemuExitCode::Success),
        "assemble" => set_state(STATE_ASSEMBLE),
        _ => return
    }
}

fn get_args(s : &str) -> Vec<&str> {
    s.split(' ').collect()
}

#[allow(dead_code)]
struct Block {
    bytes : [usize ; 16]
}

lazy_static! {
    static ref SOURCE_LINES : Mutex<TextArea> = Mutex::new(TextArea::new(
            0,
            1, 
            22, 
            80, 
            (Color::White, Color::Blue)
        )
    );
}

lazy_static! {
    static ref CURRENT_LINE : Mutex<String> = Mutex::new(String::new());
}

lazy_static! {
    static ref SCROLL_Y : Mutex<usize> = Mutex::new(0);
}

fn state_assemble() {
    let mut lines = &mut SOURCE_LINES.lock();
    let mut current_line = &mut *CURRENT_LINE.lock();

    gfx::clear(Color::Blue);
    gfx::draw_string!(0,0, (Color::White, Color::Blue), "TINIX Assembler V1.0");

    lines.draw_self();

    gfx::draw_string!(0, lines.size() + 1, (Color::White, Color::Blue), "[{:02}]>{}",lines.size() + 1,current_line.to_string());


    if let Some(chr) = tinix::stdin().read() {
        if chr == '\n' {
            lines.append_line(current_line.to_owned());
            if lines.size() > 22 {*SCROLL_Y.lock() += 1}
            current_line.clear();
            lines.set_index(*SCROLL_Y.lock());
        } else if chr == '\u{8}' {
            current_line.pop();
        } else {
            current_line.push(chr)
        }
    }

    gfx::swap();
}

fn state_command_wait() {
    let default_color = (Color::White, Color::Blue);
    let mut command_string : &mut String = &mut COMMAND.lock();
    gfx::clear(Color::Blue);
    gfx::draw_string(0,0,"==== TINIX V0.2.0a ====",default_color);
    gfx::draw_string!(0,1, default_color, "Shell V0.1.0a");
    gfx::draw_string!(0,2, default_color, "Screen Resolution: {}...",gfx::Blue::new("80x25"));
    gfx::draw_string(0,4,"=======================",default_color);
    //gfx::draw_string!(0,7,default_color, "FS Read \"{}\"[{}]: {}","/dev/null" ,stream.get_index(),stream.next());

    gfx::draw_string!(0,6,default_color,"Uptime: {} Seconds ({:.02} Minutes)",global_timer::get_seconds(), global_timer::get_minutes());

    gfx::draw_string!(0,24,default_color,">> {}",command_string);

    if let Some(chr) = tinix::stdin().read() {
        tinix::serial_println!("Character: {:?}",chr);
        if chr == '\n' {
            process_command(&command_string);
            command_string.clear();
        } else if chr == '\u{8}' {
            command_string.pop();
        } else {
            command_string.add_assign(chr.to_string().as_str())
        }
    }

    unsafe {FRAME_NUM += 1; }//FRAME_NUM %= 16}
    gfx::swap();
}









