use fbgl::renderers::{BufferedRenderer, GraphicsRenderer};

cfg_if::cfg_if! {
	if #[cfg(feature = "sdl")] {
		use fbgl::renderers::sdl::SdlRenderer;
		use sdl2::event::Event;
		use sdl2::keyboard::Keycode;
	} else {
		use fbgl::renderers::heap::HeapBuffer;
		use fbgl::renderers::fb::DirectFramebufferRenderer;
		use framebuffer::Framebuffer;
	}
}

use std::time::Duration;

use screen_hat_rs::balls::BallAnimation;
use screen_hat_rs::Animation;

fn main() {
	#[cfg(feature = "sdl")]
	let mut gl = SdlRenderer::new(960, 320).unwrap();

	#[cfg(not(feature = "sdl"))]
	let mut gl = HeapBuffer::new(
		DirectFramebufferRenderer::<fbgl::colors::Color565>::new(Framebuffer::new("/dev/fb0").unwrap()).unwrap(),
	);

	println!(
		"Framebuffer fb0 initialized as {}x{}!",
		gl.get_width(),
		gl.get_height()
	);

	let mut anim = BallAnimation::default();

	anim.init(&mut gl);

	cfg_if::cfg_if! {
		if #[cfg(feature = "sdl")] {
			let mut event_pump = gl.context.event_pump().unwrap();
			'running: loop {
				for event in event_pump.poll_iter() {
					match event {
						Event::Quit { .. }
						| Event::KeyDown {
							keycode: Some(Keycode::Escape),
							..
						} => break 'running,
						_ => {}
					}
				}
				anim.draw(&mut gl);
				gl.push_buffer();
				::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
			}
		} else {
			loop {
				anim.draw(&mut gl);
				gl.push_buffer();
				::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
			}
		}
	}
}
