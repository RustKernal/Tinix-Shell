use tinix::gfx;
use tinix::gfx::drawables::{Drawable};
use tinix::gfx::widgets::ProgressBar;
use tinix::gfx::vga::Color;
use tinix;
use tinix::interrupts::global_timer;

use alloc::{
    vec::Vec,
    string::String
};

use num_format::{Buffer, Locale};

static mut FRAME_NUM : usize = 0;

static mut _V1 : Vec<Block> = Vec::new();


#[allow(dead_code)]
pub fn shell_task() -> ! {
    let mut _data : u8 = 0;
    let mut _name : String = String::new();

    let default_color = (Color::White, Color::Blue);
    tinix::set_tick_rate(1000);
    let mut buf = Buffer::default();
    buf.write_formatted(&super::get_size(), &Locale::en);
    let s = buf.as_str();

    let mut progress = ProgressBar::new(0,5,Color::White,0,256,16);

    loop {
    gfx::clear(Color::Blue);
    gfx::draw_string(0,0,"==== TINIX V0.2.0a ====",default_color);
    gfx::draw_string!(0,1, default_color, "Shell V0.1.0a");
    gfx::draw_string!(0,2, default_color, "Screen Resolution: {}...",gfx::Blue::new("80x25"));
    gfx::draw_string!(0,3,default_color,"Heap Size: {}B", s);
    gfx::draw_string(0,4,"=======================",default_color);
    gfx::draw_string!(0,7,default_color, "FS Read \"{}\": {}",_name ,_data);
    progress.set_value(unsafe {FRAME_NUM});
    progress.set_text_color(default_color);
    progress.draw_self();

    gfx::draw_string!(0,6,default_color,"Uptime: {} Seconds ({} Minutes)",global_timer::get_seconds(), global_timer::get_minutes());

    unsafe {FRAME_NUM += 1; }//FRAME_NUM %= 16}
    gfx::swap();
    tinix::pause(16);
    }
}

#[allow(dead_code)]
struct Block {
    bytes : [usize ; 16]
}





