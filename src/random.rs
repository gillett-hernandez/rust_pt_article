use math::Vec3;
use std::f32::{consts::PI, INFINITY};

#[derive(Debug)]
pub struct Sample1D {
    pub x: f32,
}

impl Sample1D {
    pub fn new(x: f32) -> Self {
        debug_assert!(x < 1.0 && x >= 0.0);
        Sample1D { x }
    }
    pub fn new_random_sample() -> Self {
        Sample1D::new(rand::random::<f32>())
    }
    pub fn choose<T>(mut self, split: f32, a: T, b: T) -> (Self, T) {
        debug_assert!(0.0 <= split && split <= 1.0);
        debug_assert!(self.x >= 0.0 && self.x < 1.0);
        if self.x < split {
            assert!(split > 0.0);
            self.x /= split;
            (self, a)
        } else {
            // if split was 1.0, there's no way for self.x to be greather than or equal to it
            // since self.x in [0, 1)
            debug_assert!(split < 1.0);
            self.x = (self.x - split) / (1.0 - split);
            (self, b)
        }
    }
}

#[derive(Debug)]
pub struct Sample2D {
    pub x: f32,
    pub y: f32,
}

impl Sample2D {
    pub fn new(x: f32, y: f32) -> Self {
        debug_assert!(x < 1.0 && x >= 0.0);
        debug_assert!(y < 1.0 && y >= 0.0);

        Sample2D { x, y }
    }
    pub fn new_random_sample() -> Self {
        Sample2D::new(rand::random(), rand::random())
    }
}

#[derive(Debug)]
pub struct Sample3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Sample3D {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Sample3D { x, y, z }
    }
    pub fn new_random_sample() -> Self {
        Sample3D::new(rand::random(), rand::random(), rand::random())
    }
}

pub fn random_on_unit_sphere(r: Sample2D) -> Vec3 {
    // let u = 1.0 - 2.0 * r.x;
    // let sqrt1u2 = (1.0 - u * u).sqrt();
    // let (mut y, mut x) = (2.0 * PI * r.y).sin_cos();
    // x *= sqrt1u2;
    // y *= sqrt1u2;
    // Vec3::new(x, y, u)
    // let Sample2D { u, v } = self;
    let Sample2D { x, y } = r;

    let phi = x * 2.0 * PI;
    let z = y * 2.0 - 1.0;
    let r = (1.0 - z * z).sqrt();

    let (s, c) = phi.sin_cos();

    Vec3::new(r * c, r * s, z)
}

pub fn random_in_unit_disk(r: Sample2D) -> Vec3 {
    let u: f32 = r.x * PI * 2.0;
    let v: f32 = r.y.powf(1.0 / 2.0);
    Vec3::new(u.cos() * v, u.sin() * v, 0.0)
}

pub fn random_cosine_direction(r: Sample2D) -> Vec3 {
    let Sample2D { x: u, y: v } = r;
    let z: f32 = (1.0 - v).sqrt();
    let phi: f32 = 2.0 * PI * u;
    let (mut y, mut x) = phi.sin_cos();
    x *= v.sqrt();
    y *= v.sqrt();
    Vec3::new(x, y, z)
}

pub fn weighted_cosine_direction(r: Sample2D, weight: f32) -> Vec3 {
    let Sample2D { x: u, y: v } = r;
    let z: f32 = weight * (1.0 - v).sqrt();
    let phi: f32 = 2.0 * PI * u;
    let (mut y, mut x) = phi.sin_cos();
    x *= v.sqrt();
    y *= v.sqrt();
    Vec3::new(x, y, z).normalized()
}

pub fn random_to_sphere(r: Sample2D, radius: f32, distance_squared: f32) -> Vec3 {
    let r1 = r.x;
    let r2 = r.y;
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);
    let phi = 2.0 * PI * r1;
    let (mut y, mut x) = phi.sin_cos();
    let sqrt_1_z2 = (1.0 - z * z).sqrt();
    x *= sqrt_1_z2;
    y *= sqrt_1_z2;
    return Vec3::new(x, y, z);
}

pub trait Sampler {
    fn draw_1d(&mut self) -> Sample1D;
    fn draw_2d(&mut self) -> Sample2D;
    fn draw_3d(&mut self) -> Sample3D;
}

pub struct RandomSampler {}

impl RandomSampler {
    pub const fn new() -> RandomSampler {
        RandomSampler {}
    }
}

impl Sampler for RandomSampler {
    fn draw_1d(&mut self) -> Sample1D {
        Sample1D::new_random_sample()
    }
    fn draw_2d(&mut self) -> Sample2D {
        Sample2D::new_random_sample()
    }
    fn draw_3d(&mut self) -> Sample3D {
        Sample3D::new_random_sample()
    }
}
