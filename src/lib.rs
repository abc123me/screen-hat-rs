use fbgl::image::ImageOperations;
use fbgl::renderers::GraphicsOperations;

pub mod balls;
pub mod nyan;

pub trait Animation {
	fn init<T: GraphicsOperations + ImageOperations>(&mut self, gl: &mut T) -> ();
	fn draw<T: GraphicsOperations + ImageOperations>(&mut self, gl: &mut T) -> ();
}
