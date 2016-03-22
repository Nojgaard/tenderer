extern crate num;
use std::ops::{Add,Sub,Mul};
use self::num::{Num, ToPrimitive, NumCast};

#[derive(Copy,Clone)]
pub struct Vec3<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T
}

#[derive(Copy,Clone)]
pub struct Vec2<T: Num> {
    pub x: T,
    pub y: T
}

impl<T: Num> Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2::new(self.x+rhs.x,self.y+rhs.y)
    }
}

impl<T: Num> Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn sub(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2::new(self.x-rhs.x,self.y-rhs.y)
    }
}

impl<T: Num + NumCast + ToPrimitive> Mul<f32> for Vec2<T> {
    type Output = Vec2<T>;
    fn mul(self, rhs: f32) -> Vec2<T> {
        let x = self.x.to_f32().unwrap();
        let x: T = NumCast::from(x * rhs).unwrap();
        let y = self.y.to_f32().unwrap();
        let y: T = NumCast::from(y * rhs).unwrap();
        Vec2::new(x,y)
    }
}

impl<T: Num + Copy + Clone + ToPrimitive> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 {x:x, y:y, z:z}
    }

    pub fn cross(&self, v2: Vec3<T>) -> Vec3<T> {
        let v1 = &self;
        Vec3::new(v1.y*v2.z-v1.z*v2.y, v1.z*v2.x-v1.x*v2.z, v1.x*v2.y-v1.y*v2.x)
    }

    pub fn dot(&self, v2: Vec3<T>) -> T {
        let v1 = &self;
        v1.x*v2.x+v1.y*v2.y+v1.z*v2.z
    }

    pub fn norm(&self) -> f32 {
        let x = self.x.to_f32().unwrap();
        let y = self.y.to_f32().unwrap();
        let z = self.z.to_f32().unwrap();
        (x*x+y*y+z*z).sqrt()
    }

    pub fn normalize(&self) -> Vec3<f32> {
        let x = self.x.to_f32().unwrap();
        let y = self.y.to_f32().unwrap();
        let z = self.z.to_f32().unwrap();
        Vec3::new(x,y,z)*(1./self.norm())
    }
}

impl<T: Num + Copy + Clone + ToPrimitive> Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn sub(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x-rhs.x,self.y-rhs.y, self.z-rhs.z)
    }
}

impl<T: Num + NumCast + ToPrimitive + Copy + Clone> Mul<f32> for Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, rhs: f32) -> Vec3<T> {
        let x = self.x.to_f32().unwrap();
        let x: T = NumCast::from(x * rhs).unwrap();
        let y = self.y.to_f32().unwrap();
        let y: T = NumCast::from(y * rhs).unwrap();
        let z = self.z.to_f32().unwrap();
        let z: T = NumCast::from(z * rhs).unwrap();
        Vec3::new(x,y,z)
    }
}

impl<T: Num> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 {x:x, y:y }
    }

    pub fn new_pair(v: (T,T)) -> Vec2<T> {
        let (x,y) = v;
        Vec2::new(x,y)
    }

}

pub fn barycentric<T>(v: [Vec2<i32>;3], p: Vec2<i32>) -> Vec3<f32> {
    let a = Vec3::new((v[2].x-v[0].x) as f32, (v[1].x-v[0].x) as f32, (v[0].x-p.x) as f32);
    let b = Vec3::new((v[2].y-v[0].y) as f32, (v[1].y-v[0].y) as f32, (v[0].y-p.y) as f32);
    let u = a.cross(b);
    if u.z.abs() < 1. {return Vec3::new(-1.,1.,1.);}
    Vec3::new(1.-(u.x+u.y)/u.z, u.y/u.z, u.x/u.z)
}
