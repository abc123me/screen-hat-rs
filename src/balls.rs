use crate::Animation;

use rand::{rngs::ThreadRng, RngExt};

use fbgl::colors::ReprColor;
use fbgl::renderers::{GraphicsOperations, GraphicsRenderer};

const BALL_CNT: usize = 25;

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
			<T as GraphicsRenderer>::Color::new(255, 255, 255),
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

pub struct BallAnimation {
	balls: Vec<Ball>,
	width: u32,
	height: u32,
	rng: ThreadRng,
}

impl Default for BallAnimation {
	fn default() -> Self {
		Self {
			rng: rand::rng(),
			balls: Vec::with_capacity(BALL_CNT),
			width: 0,
			height: 0,
		}
	}
}

impl Animation for BallAnimation {
	fn init<T: GraphicsOperations>(&mut self, gl: &mut T) {
		gl.clear(<T as GraphicsRenderer>::Color::new(0, 0, 0));
		self.width = gl.get_width();
		self.height = gl.get_height();
		for _i in 0..BALL_CNT {
			let sz = self.rng.random_range(15..50) as u32;
			let ball = Ball {
				vx: self.rng.random_range(-10..10),
				vy: self.rng.random_range(-10..10),
				xp: self.rng.random_range(sz..(self.width - sz)),
				yp: self.rng.random_range(sz..(self.height - sz)),
				sz,
			};
			//println!("Ball at {}, {} - size {}", ball.xp, ball.yp, ball.sz);
			self.balls.push(ball);
		}
	}
	fn draw<T: GraphicsOperations>(&mut self, gl: &mut T) {
		for ball in &mut self.balls {
			ball.physics(self.width, self.height);
		}

		gl.clear(<T as GraphicsRenderer>::Color::new(0, 0, 0));
		for ball in &self.balls {
			ball.draw(gl);
		}
	}
}
