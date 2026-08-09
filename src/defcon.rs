use fbgl::colors::ReprColor;
use fbgl::image::ImageOperations;
use fbgl::renderers::{GraphicsOperations, GraphicsRenderer};
use fbgl::text::{TextOperations, TextRenderSettings};

use image::{ImageBuffer, Rgba};

use crate::{load_image_resize, Animation};

pub struct DefconAnimation {
	width: u32,
	height: u32,
	img1: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
	pos : u32
}

impl Default for DefconAnimation {
	fn default() -> Self {
		Self {
			img1: None,
			width: 0,
			height: 0,
			pos: 0,
		}
	}
}

impl Animation for DefconAnimation {
	fn init<T: GraphicsOperations + ImageOperations + TextOperations>(&mut self, gl: &mut T) {
		gl.clear(<T as GraphicsRenderer>::Color::new(0, 0, 0));
		self.width = gl.get_width();
		self.height = gl.get_height();
		self.img1 = Some(load_image_resize("defcon34.jpg", self.height, self.height));
	}
	fn draw<T: GraphicsOperations + ImageOperations + TextOperations>(&mut self, gl: &mut T) {
		gl.clear(<T as GraphicsRenderer>::Color::new(0, 0, 0));
		if let Some(img) = &self.img1 {
			let th = self.height - 50;
			let txt = "DC34";
			let trs = TextRenderSettings {
				font: fontdue::Font::from_bytes(
					include_bytes!("/usr/share/fonts/noto/NotoSans-Bold.ttf") as &[u8],
					fontdue::FontSettings {
						scale: th as f32,
						..fontdue::FontSettings::default()
					},
				)
				.unwrap(),
				size: th,
				blend: false,
			};
			let (sz_w, sz_h) = gl.text_size(txt, &trs);
			let pos0 = self.pos % self.width;
			self.pos = pos0;
			let pos1 = pos0 + self.height;
			let pos2 = pos1 + sz_w;

			gl.draw_image_rgba(pos0, 0, img);
			gl.text(
				<T as GraphicsRenderer>::Color::new(255, 0, 255),
				pos1,
				0,
				txt.to_string(),
				trs,
			);
			gl.draw_image_rgba(pos2, 0, img);

			self.pos += 5;
		}
	}
}
