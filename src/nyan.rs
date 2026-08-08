use fbgl::colors::ReprColor;
use fbgl::image::sprite::{AnimatedSprite, SpriteSheetFormat};
use fbgl::image::ImageOperations;
use fbgl::renderers::{GraphicsOperations, GraphicsRenderer};

use image::{ImageBuffer, Rgba};

use rand::{rngs::ThreadRng, RngExt};

use std::time::Duration;

use crate::{load_image, Animation};

const NORM_CAT_COUNT: usize = 3;
const CAPE_CAT_COUNT: usize = 2;

#[derive(Copy, Clone, PartialEq)]
enum NyanStyle {
	Normal,
	Cape,
}

fn make_sprite(
	img: &ImageBuffer<Rgba<u8>, Vec<u8>>,
	c: u32,
	w: u32,
	h: u32,
	x: u32,
	y: u32,
	t: u32,
) -> AnimatedSprite {
	AnimatedSprite::load_from_image(
		&img,
		SpriteSheetFormat::Horizontal,
		c,
		w,
		h,
		x,
		y,
		Duration::from_millis(t.into()),
	)
}

impl NyanStyle {
	fn cat_width(self) -> u32 {
		match self {
			NyanStyle::Normal => 150,
			NyanStyle::Cape => 80,
		}
	}
	fn rainbow_width(self) -> u32 {
		match self {
			NyanStyle::Normal => 135,
			NyanStyle::Cape => 180,
		}
	}
	fn rainbow_offset(self) -> u32 {
		match self {
			NyanStyle::Normal => 90,
			NyanStyle::Cape => 90,
		}
	}
	fn average_speed(self) -> i32 {
		(self.rainbow_width() / 20) as i32
	}
	fn load_cat_sprite(self, img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> AnimatedSprite {
		match self {
			NyanStyle::Normal => make_sprite(img, 6, self.cat_width(), 100, 0, 0, 50),
			NyanStyle::Cape => make_sprite(img, 4, self.cat_width(), 40, 0, 220, 75),
		}
	}
	fn load_rainbow_sprite(self, img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> AnimatedSprite {
		match self {
			NyanStyle::Normal => make_sprite(img, 2, self.rainbow_width(), 100, 910, 0, 155),
			NyanStyle::Cape => make_sprite(img, 2, self.rainbow_width(), 40, 330, 400, 75),
		}
	}
	fn total_width(self) -> u32 {
		self.cat_width() + self.rainbow_width()
	}
	fn total_height(self) -> u32 {
		match self {
			NyanStyle::Normal => 100,
			NyanStyle::Cape => 40,
		}
	}
	fn load_sprite(self, img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> NyanSprite {
		NyanSprite {
			cat: self.load_cat_sprite(img),
			rainbow: self.load_rainbow_sprite(img),
			style: self,
			x: 0,
			y: 0,
			vx: self.average_speed(),
			vy: 0,
		}
	}
}

struct NyanSprite {
	cat: AnimatedSprite,
	rainbow: AnimatedSprite,
	style: NyanStyle,
	x: i32,
	y: i32,
	vx: i32,
	vy: i32,
}

impl NyanSprite {
	fn init(&mut self) {
		self.rainbow.reset_animation();
		self.cat.reset_animation();
	}
	fn draw(&mut self, gl: &mut impl ImageOperations) {
		self.rainbow.draw_sprite(gl, self.x as u32, self.y as u32);
		self.cat.draw_sprite(
			gl,
			self.x as u32 + self.style.rainbow_offset(),
			self.y as u32,
		);
	}
	fn physics(&mut self, screen_w: i32, screen_h: i32) {
		// Casually bounce off top/bottom walls
		let new_y = self.y + self.vy;
		if (new_y + self.style.total_height() as i32) >= screen_h || new_y < 0 {
			self.vy = -self.vy;
		}
		self.y = self.y + self.vy;

		// Just wrap on x overflow
		let new_x = self.x + self.vx;
		self.x = new_x % screen_w;
	}
}

pub struct NyanAnimation {
	cats: Vec<NyanSprite>,
	width: u32,
	height: u32,
	rng: ThreadRng,
}

impl Default for NyanAnimation {
	fn default() -> Self {
		Self {
			rng: rand::rng(),
			cats: Vec::with_capacity(NORM_CAT_COUNT + CAPE_CAT_COUNT),
			width: 0,
			height: 0,
		}
	}
}

impl Animation for NyanAnimation {
	fn init<T: GraphicsOperations + ImageOperations>(&mut self, gl: &mut T) {
		gl.clear(<T as GraphicsRenderer>::Color::new(0, 0, 0));
		self.width = gl.get_width();
		self.height = gl.get_height();
		let img = load_image("nyan.png");
		let rng = &mut self.rng;
		for _ in 0..NORM_CAT_COUNT {
			self.cats.push(NyanStyle::Normal.load_sprite(&img));
		}
		for _ in 0..CAPE_CAT_COUNT {
			self.cats.push(NyanStyle::Cape.load_sprite(&img));
		}
		self.cats.iter_mut().for_each(|cat| {
			let ch = cat.style.total_height();
			let cw = cat.style.total_width() * 2;
			let ca = cat.style.average_speed();
			cat.init();
			cat.vy = rng.random_range(-5..5);
			cat.y = rng.random_range(0..(self.height as i32 - ch as i32));
			cat.x = rng.random_range(0..(self.width as i32 - cw as i32));
			cat.vx += rng.random_range((ca / 4)..(ca * 3));
		});
	}
	fn draw<T: GraphicsOperations + ImageOperations>(&mut self, gl: &mut T) {
		self.cats
			.iter_mut()
			.for_each(|cat| cat.physics(self.width as i32, self.height as i32));
		gl.clear(<T as GraphicsRenderer>::Color::new(0, 0, 0));
		self.cats.iter_mut().for_each(|cat| cat.draw(gl));
	}
}
