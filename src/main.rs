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

use std::time::{Instant, Duration};

use screen_hat_rs::balls::BallAnimation;
use screen_hat_rs::nyan::NyanAnimation;
use screen_hat_rs::Animation;

const NEXT_AFTER: Duration = Duration::from_secs(30);
const FRAME_TIME: Duration = Duration::from_millis(16);

enum SelectedAnimation {
	Ball,
	Nyan,
}

fn main() {
	#[cfg(feature = "sdl")]
	let mut gl = SdlRenderer::new(960, 320).unwrap();

	#[cfg(not(feature = "sdl"))]
	let mut gl = HeapBuffer::new(
		DirectFramebufferRenderer::<fbgl::colors::Color565>::new(
			Framebuffer::new("/dev/fb0").unwrap(),
		)
		.unwrap(),
	);

	println!(
		"Framebuffer fb0 initialized as {}x{}!",
		gl.get_width(),
		gl.get_height()
	);

	let mut ball = BallAnimation::default();
	ball.init(&mut gl);

	let mut nyan = NyanAnimation::default();
	nyan.init(&mut gl);

	let mut sel = SelectedAnimation::Ball;

	let mut frame_time = Instant::now() + FRAME_TIME;
	let mut nexta_time = Instant::now() + NEXT_AFTER;

	#[cfg(feature = "sdl")]
	let mut event_pump = gl.context.event_pump().unwrap();

	'running: loop {
		#[cfg(feature = "sdl")]
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. }
				| Event::KeyDown {
					keycode: Some(Keycode::Escape),
					..
				} => break 'running,
				Event::KeyDown {
					keycode: Some(Keycode::Space),
					..
				} => {
					nexta_time = Instant::now() + NEXT_AFTER;
					sel = sel.next();
				}
				_ => {}
			}
		}

		let ctime = Instant::now();
		if ctime > frame_time {
			if ctime > nexta_time {
				nexta_time = Instant::now() + NEXT_AFTER;
				sel = sel.next();
			}

			frame_time = Instant::now() + FRAME_TIME;
			match sel {
				SelectedAnimation::Nyan => nyan.draw(&mut gl),
				SelectedAnimation::Ball => ball.draw(&mut gl),
			};
			gl.push_buffer();
		} else {
			::std::thread::sleep(Duration::from_millis(1));
		}
	}
}

impl SelectedAnimation {
	fn next(self) -> SelectedAnimation {
		use SelectedAnimation::*;
		match self {
			Ball => Nyan,
			Nyan => Ball,
		}
	}
}
