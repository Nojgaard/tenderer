use std::fs::File;
use std::io::{BufReader, BufRead};
use geometry::{Vec3};

// pub struct Point{
//     pub x: f32,
//     pub y: f32,
//     pub z: f32
// }

pub struct Model {
    pub verts: Vec<Vec3<f32>>,
    pub faces: Vec<Vec<usize>>
}

impl Model {
    pub fn new(path: &str) -> Model {
        let mut m = Model{verts: vec![], faces: vec![vec![]]};
        let f = BufReader::new(File::open(path).unwrap());
        for line in f.lines().filter_map(|x| x.ok()) {
            if line.starts_with("v ") {
                let mut vi = line.split(" ").filter_map(|s| s.parse::<f32>().ok());
                m.verts.push(Vec3::new(
                        vi.next().unwrap(), 
                        vi.next().unwrap(), 
                        vi.next().unwrap()));
            } else if line.starts_with("f ") {
                let mut f: Vec<usize> = vec![];
                let mut i = line.split(" ");
                i.next();
                while let Some(x) = i.next() {
                    f.push(x.split("/").next().unwrap().parse::<usize>().unwrap()-1);
                }
                m.faces.push(f);
            }
        }
        println!("v: {}, f: {}",m.verts.len(), m.faces.len());
        m
    }
}
