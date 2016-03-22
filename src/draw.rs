use super::Image;
use super::image::Rgba;
use super::colors::Color;
use super::colors;
use super::geometry::{Vec2,Vec3, barycentric};
use super::model::Model;
// use std::mem;
use std::cmp::{min,max};

pub fn line(v0: Vec2<i32>, v1: Vec2<i32>, img: &mut Image, color: Color) {
    let color = Rgba(color);
    let (x0,y0,x1,y1,steep) = 
        if (v0.x-v1.x).abs() < (v0.y-v1.y).abs() {
            (v0.y,v0.x,v1.y,v1.x,true)
        } else {
            (v0.x,v0.y,v1.x,v1.y,false)
        };

    let (x0,y0,x1,y1) = if x0 > x1 {(x1,y1,x0,y0)} else {(x0,y0,x1,y1)};

    let (dx,dy) = ((x1-x0),(y1-y0));
    let derror = dy.abs()*2;
    let mut error = 0;
    let mut y = y0;
    for x in x0..(x1+1) {
        if steep {
            img.put_pixel(y as u32,x as u32,color);
        } else {
            img.put_pixel(x as u32,y as u32,color);
        }
        error += derror;
        if error > dx {
            y += if y1 > y0 {1} else {-1};
            error -= dx*2;
        }
    }
}

// pub fn triangleOld(mut v0: Vec2<i32>, mut v1: Vec2<i32>, mut v2: Vec2<i32>
//                 , img: &mut Image, color: Color) {
//     if v0.y == v1.y && v0.y == v2.y {return;}
//     if v0.y > v1.y {mem::swap(&mut v0, &mut v1);}
//     if v0.y > v2.y {mem::swap(&mut v0, &mut v2);}
//     if v1.y > v2.y {mem::swap(&mut v1, &mut v2);}
//     let total_height = v2.y - v0.y;
//     for y in 0..(total_height+1) {
//         let second_half = y >= (v1.y-v0.y);
//         let segment_height = if second_half {v2.y-v1.y} else {v1.y-v0.y};
//         let alpha = y as f32 / total_height as f32;
//         // let beta  = (y-v0.y) as f32 / segment_height as f32;
//         let beta = (y-if second_half {v1.y-v0.y} else {0}) as f32 / segment_height as f32;
//         let mut A = v0 + (v2-v0)*alpha;
//         let mut B = if second_half {v1+(v2-v1)*beta} else {v0 + (v1-v0)*beta};
//         if A.x > B.x {mem::swap(&mut A, &mut B);}
//         for x in A.x..(B.x+1) {
//             // println!("{} {} {}", x, y,A.x);
//             img.put_pixel(x as u32, (v0.y+y) as u32, Rgba(color));
//         }
//     }
//     // line(v0,v1,img,colors::WHITE);
//     // line(v1,v2,img,colors::WHITE);
//     // line(v2,v0,img,colors::GREEN);
// }
pub fn triangle(v: [Vec2<i32>;3], img: &mut Image, color: Color) {
    let (w,h) = img.dimensions();
    let (w,h) = (w as i32, h as i32);
    let mut bboxmin = Vec2::new(w-1,h-1);
    let mut bboxmax = Vec2::new(0,0);
    let clamp = Vec2::new(w-1,h-1);
    for &vi in v.iter() {
        bboxmin.x = max(0, min(bboxmin.x, vi.x));
        bboxmax.x = min(clamp.x, max(bboxmax.x, vi.x));
        bboxmin.y = max(0, min(bboxmin.y, vi.y));
        bboxmax.y = min(clamp.y, max(bboxmax.y, vi.y));
    }
    for x in bboxmin.x..bboxmax.x {
        for y in bboxmin.y..bboxmax.y {
            let p = Vec2::new(x,y);
            let bc = barycentric::<i32>(v, p);
            if bc.x < 0. || bc.y < 0. || bc.z < 0. {continue;}
            img.put_pixel(p.x as u32, p.y as u32, Rgba(color));
        }
    }
}

pub fn model(m: &Model, img: &mut Image, color: Color) {
    let (w,h) = img.dimensions();
    let (w,h) = (w-1,h-1);
    for f in m.faces.iter() {
        if f.len() == 0 {continue;}
        let mut coords = [Vec2::new(0,0); 3];
        let mut wv = [Vec3::new(0.,0.,0.);3];
        for (i, v) in coords.iter_mut().enumerate() {
            wv[i] = m.verts[f[i]];
            v.x = ((wv[i].x+1.)*(w as f32/2.)) as i32;
            v.y = ((wv[i].y+1.)*(h as f32/2.)) as i32;
        }
        let n = (wv[2]-wv[0]).cross(wv[1]-wv[0]).normalize();
        let light_dir = Vec3::new(0.,0.,-1.);
        let intensity = n.dot(light_dir); 
        if intensity > 0. {
            triangle(coords, img, colors::gray_scale(intensity));
        }
        // triangle(coords, img,
        // for i in 0..3 {
        //     let v0 = &m.verts[f[i]];
        //     let v1 = &m.verts[f[(i+1)%3]];
        //     let x0 = ((v0.x+1.)*(w as f32/2.)) as i32;
        //     let y0 = ((v0.y+1.)*(h as f32/2.)) as i32;
        //     let x1 = ((v1.x+1.)*(w as f32/2.)) as i32;
        //     let y1 = ((v1.y+1.)*(h as f32/2.)) as i32;
        //     line(Vec2::new(x0,y0), Vec2::new(x1,y1), img, color);
        // }
    }
}
