use fbgl::fb::*;
use fbgl::*;

use framebuffer::{Framebuffer, KdMode};

use rand::RngExt;

struct Ball {
    xp: u32,
    yp: u32,
    sz: u32,
    vx: i32,
    vy: i32,
}

impl Ball {
    fn draw<T: GraphicsOperations>(&self, gl: &mut T) {
        gl.ellipse(
            <T as fbgl::GraphicsRenderer>::Color::new(255, 255, 255),
            self.xp,
            self.yp,
            self.sz,
            self.sz,
        );
    }
    fn physics(&mut self, w: u32, h: u32) {
        let isz = self.sz as i32;
        let nxp = self.xp as i32 + self.vx;
        let nyp = self.yp as i32 + self.vy;
        if nxp + isz > w as i32 || nxp - isz < 0 {
            self.vx = -self.vx;
        }
        if nyp + isz > h as i32 || nyp - isz < 0 {
            self.vy = -self.vy;
        }
        self.xp = (self.xp as i32 + self.vx) as u32;
        self.yp = (self.yp as i32 + self.vy) as u32;
    }
}

fn main() {
    let gfx_mode = Framebuffer::set_kd_mode(KdMode::Graphics);
    if !gfx_mode.is_ok() {
        println!("Failed to set graphics mode on framebuffer!");
    }

    let mut gl = BufferedRenderer::new(MultiDisplayHorizontalRenderer::<
        DirectFramebufferRenderer<Color565>,
        6,
    >::new([
        //DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb0").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb1").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb2").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb3").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb4").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb5").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb6").unwrap()).unwrap(),
        //DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb7").unwrap()).unwrap(),
        /*DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb8").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb9").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb10").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb11").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb12").unwrap()).unwrap(),
        DirectFramebufferRenderer::<Color565>::new(Framebuffer::new("/dev/fb13").unwrap()).unwrap(),*/
    ]));

    println!(
        "Framebuffer fb0 initialized as {}x{}!",
        gl.get_width(),
        gl.get_height()
    );

    gl.clear(Color565::new(0, 0, 0));

    let (w, h) = (gl.get_width(), gl.get_height());

    let mut rng = rand::rng();

    const ball_cnt: usize = 5;
    let mut balls = Vec::with_capacity(ball_cnt);

    for i in 0..ball_cnt {
        let sz = rng.random_range(15..50) as u32;
        balls.push(Ball {
            vx: rng.random_range(-10..10),
            vy: rng.random_range(-10..10),
            xp: rng.random_range(sz..(w - sz)) as u32,
            yp: rng.random_range(sz..(h - sz)) as u32,
            sz,
        });
    }

    loop {
        for ball in &mut balls {
            ball.physics(w, h);
        }

        gl.clear(Color565::new(0, 0, 0));
        for ball in &balls {
            ball.draw(&mut gl);
        }
        gl.push_buffer();
    }

    if gfx_mode.is_ok() {
        let _ = Framebuffer::set_kd_mode(KdMode::Text);
    }
}
