use fbgl::image::ImageOperations;
use fbgl::renderers::GraphicsOperations;

use image::{ImageBuffer, ImageReader, DynamicImage, Rgba, imageops::FilterType};

pub mod balls;
pub mod defcon;
pub mod nyan;
pub mod npixel;

pub trait Animation {
	fn init<T: GraphicsOperations + ImageOperations>(&mut self, gl: &mut T) -> ();
	fn draw<T: GraphicsOperations + ImageOperations>(&mut self, gl: &mut T) -> ();
}

pub fn load_image_raw(name: &str) -> DynamicImage {
	match ImageReader::open(format!("/usr/share/screen-hat/{name}")) {
		Ok(img) => img,
		Err(err) => {
			eprintln!("Unable to open /usr/share/screen-hat/{name} trying assets/{name}! {err}");
			ImageReader::open(format!("assets/{name}")).expect("able to open assets/{name}")
		}
	}
	.decode()
	.expect(format!("able to decode {name} spritesheet").as_str())
}


pub fn load_image(name: &str) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	load_image_raw(name).to_rgba8()
}

pub fn load_image_resize(name: &str, new_w: u32, new_h: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	load_image_raw(name).resize_exact(new_w, new_h, FilterType::Triangle).to_rgba8()
}

pub mod animations {
	pub use crate::balls::BallAnimation;
	pub use crate::defcon::DefconAnimation;
	pub use crate::nyan::NyanAnimation;
	pub use crate::npixel::NuclearPixelAnimation;
}
