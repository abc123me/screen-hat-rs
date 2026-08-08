use fbgl::colors::ReprColor;
use fbgl::image::ImageOperations;
use fbgl::renderers::{GraphicsOperations, GraphicsRenderer};

use image::{ImageBuffer, Rgba};

use crate::{load_image_resize, Animation};

pub struct DefconAnimation {
	width: u32,
    height: u32,
    img1: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    img2: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
}

impl Default for DefconAnimation {
	fn default() -> Self {
        Self {
            img1: None,
            img2: None,
			width: 0,
			height: 0,
		}
	}
}

impl Animation for DefconAnimation {
	fn init<T: GraphicsOperations + ImageOperations>(&mut self, gl: &mut T) {
		gl.clear(<T as GraphicsRenderer>::Color::new(0, 0, 0));
		self.width = gl.get_width();
        self.height = gl.get_height();
        self.img1 = Some(load_image_resize("defcon34.jpg", 200, 200));
        self.img2 = Some(load_image_resize("defcon34.jpg", 200, 200));
	}
	fn draw<T: GraphicsOperations + ImageOperations>(&mut self, gl: &mut T) {
		gl.clear(<T as GraphicsRenderer>::Color::new(0, 0, 0));
		if let Some(img) = &self.img1 {
            gl.draw_image_rgba(0, 0, img);
        }
	}
}
