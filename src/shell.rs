use tinix::gfx;
use tinix::gfx::vga::Color;
use tinix;
use tinix::gfx::vga::Pixel;



pub fn shell_task() {
    let default_color = (Color::White, Color::Blue);
    gfx::clear(Pixel::from_color(Color::Blue));
    gfx::draw_string(0,0,"==== TINIX V0.1.0a ====",default_color);
    gfx::draw_string(0,1,"Screen Resolution: \x0280\x03x\x0225...",default_color);
    gfx::draw_string(0,2,"Heap Size: \x0e100KiB...",default_color);
    gfx::draw_string(0,3,"=======================",default_color);

    gfx::draw_rect(0,4,10,1, Color::Green);
    gfx::draw_string(11,4, " | 100% ", default_color);
}
