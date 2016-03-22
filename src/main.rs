extern crate tenderer;
// use tenderer::geometry::{Line,LinePairArgs};
use tenderer::geometry::Vec2;
use tenderer::draw;
use tenderer::colors;

fn main() {
    let m = tenderer::model::Model::new("data/african_head.obj");
    let (w,h) = (800,800);
    let mut img = tenderer::new_image(w,h);
    // draw::triangle([Vec2::new(10,70),Vec2::new(50,160),Vec2::new(70,80)]
    //                , &mut img, colors::RED);
    // draw::triangle([Vec2::new(180,50),Vec2::new(150,1),Vec2::new(70,180)]
    //                , &mut img, colors::RED);
    // draw::triangle([Vec2::new(180,150),Vec2::new(120,160),Vec2::new(130,180)]
    //                , &mut img, colors::RED);
    // Triangle::new((180,50),(150,1),(70,180)).draw(&mut img, colors::RED);
    // Triangle::new((180,150),(120,160),(130,180)).draw(&mut img, colors::RED);

    draw::model(&m, &mut img, colors::WHITE);
    tenderer::render("test.png",&img);
}
