use tinix::gfx;
use tinix::gfx::drawables::{Drawable};
use tinix::gfx::widgets::ProgressBar;
use tinix::gfx::vga::Color;
use tinix;
use tinix::gfx::vga::Pixel;

use alloc::{
    boxed::Box,
    vec::Vec
};

use num_format::{Buffer, Locale};

static mut frame_num : usize = 0;

static mut v1 : Vec<Block> = Vec::new();


pub fn shell_task() -> ! {
    let default_color = (Color::White, Color::Blue);
    tinix::set_tick_rate(10000);
    let mut buf = Buffer::default();
    buf.write_formatted(&super::get_size(), &Locale::en);
    let s = buf.as_str();
    loop {
    gfx::clear(Color::Blue);
    gfx::draw_string(0,0,"==== TINIX V0.2.0a ====",default_color);
    gfx::draw_string!(0,1, default_color, "Shell V0.1.0a");
    gfx::draw_string!(0,2, default_color, "Screen Resolution: {}...",gfx::Green::new("80x25"));
    gfx::draw_string!(0,3,default_color,"Heap Size: {}B", s);
    gfx::draw_string(0,4,"=======================",default_color);
    gfx::swap();
    }
}

struct Block {
    bytes : [usize ; 16]
}





