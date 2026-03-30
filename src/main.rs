use fbgl::fb::*;
use fbgl::*;

use framebuffer::{Framebuffer, KdMode};

fn main() {
    let gfx_mode = Framebuffer::set_kd_mode(KdMode::Graphics);
    if !gfx_mode.is_ok() {
        println!("Failed to set graphics mode on framebuffer!");
    }

    let mut gl = MultiDisplayHorizontalRenderer::<DirectFramebufferRenderer<Color565>, 14>::new([
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb0").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb1").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb2").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb3").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb4").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb5").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb6").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb7").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb8").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb9").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb10").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb11").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb12").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb13").unwrap()).unwrap(),
    ]);

    println!(
        "Framebuffer fb0 initialized as {}x{}!",
        gl.get_width(),
        gl.get_height()
    );
    let w = gl.get_width();
    let h = gl.get_height();
    let s = h / 2;
    let w2 = w / 2;
    let h2 = h / 2;
    let s2 = s / 2;

    gl.clear(Color565::new(125, 125, 125));

    if gfx_mode.is_ok() {
        let _ = Framebuffer::set_kd_mode(KdMode::Text);
    }
}
