extern crate rand;

use self::rand::Rng;
pub type Color = [u8;4];
pub const RED: Color = [255,0,0,255];
pub const BLACK: Color = [0,0,5,255];
pub const WHITE: Color = [255,255,255,255];
pub const GREEN: Color = [0,255,0,255];

pub fn random() -> Color {
    let mut rng = rand::thread_rng();
    [rng.gen::<u8>(),rng.gen::<u8>(),rng.gen::<u8>(),255]
}

pub fn gray_scale(scalar: f32) -> Color {
    let code = (scalar * 255.) as u8;
    [code,code,code,255]
}
