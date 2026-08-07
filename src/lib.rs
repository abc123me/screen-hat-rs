use fbgl::renderers::GraphicsOperations;

pub mod balls;

pub trait Animation {
	fn init<T: GraphicsOperations>(&mut self, gl: &mut T) -> ();
	fn draw<T: GraphicsOperations>(&mut self, gl: &mut T) -> ();
}
