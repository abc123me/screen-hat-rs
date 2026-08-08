use fbgl::colors::ReprColor;
use fbgl::image::ImageOperations;
use fbgl::renderers::{GraphicsOperations, GraphicsRenderer};

use image::{ImageBuffer, Rgba};

use crate::{load_image_resize, Animation};

use std::time::{Instant, Duration};

pub struct NuclearPixelAnimation {
    width: u32,
    height: u32,
    img_num: usize,
    load_time: Instant,
    imgs: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>,
}

impl Default for NuclearPixelAnimation {
    fn default() -> Self {
        Self {
            imgs: Vec::with_capacity(24),
            width: 0,
            height: 0,
            img_num: 0,
            load_time: Instant::now(),
        }
    }
}

impl Animation for NuclearPixelAnimation {
    fn init<T: GraphicsOperations + ImageOperations>(&mut self, gl: &mut T) {
        gl.clear(<T as GraphicsRenderer>::Color::new(0, 0, 0));
        self.width = gl.get_width();
        self.height = gl.get_height();
        for i in 1..25 {
            self.imgs.push(load_image_resize(format!("npixel-{i:04}.jpg").as_str(), self.width, self.height));
        }
    }
    fn draw<T: GraphicsOperations + ImageOperations>(&mut self, gl: &mut T) {
        gl.clear(<T as GraphicsRenderer>::Color::new(0, 0, 0));
        gl.draw_image_rgba(0, 0, self.imgs.get(self.img_num).expect("all images loaded and in bounds img_num"));
        if Instant::now() > self.load_time {
            self.load_time = Instant::now() + Duration::from_millis(50);
            self.img_num += 1;
            if self.img_num >= 24 {
                self.img_num = 0;
            }
        }
    }
}
