extern crate image;
use image::{ImageBuffer, Rgba};

pub type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub mod colors;
pub mod model;
pub mod geometry;
pub mod draw;

// use colors::Color;

pub fn new_image(width:u32,height:u32) -> Image {
    let black = Rgba(colors::BLACK);
    ImageBuffer::from_pixel(width,height,black)
}

pub fn render(path: &str, img: &Image) {
    let img = image::imageops::flip_vertical(img);
    let _ = img.save(path).unwrap();
}
