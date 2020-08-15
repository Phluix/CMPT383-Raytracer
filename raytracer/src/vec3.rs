use std::ops::{Add, Sub, Mul, Div, Neg, Index, AddAssign, MulAssign, DivAssign};

use rand::prelude::*;
use std::f32::consts::PI;

#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3 {
    e: [f32;3],
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 {
            e: [e0, e1, e2],
        }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn r(&self) -> f32 {
        self.e[0]
    }
    pub fn g(&self) -> f32 {
        self.e[1]
    }
    pub fn b(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn dot(&self, other: Vec3) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: Vec3) -> Self {
        Vec3 {
            e: [
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(),
            ]
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
         *self / self.length()
    }

    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            e: [rng.gen(), rng.gen(), rng.gen()],
        }
    }

    pub fn random_init(min: f32, max: f32) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            e: [
                rng.gen_range(min, max),
                rng.gen_range(min, max),
                rng.gen_range(min, max),
            ]
        }
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_init(-1.0, 1.0);
        if p.length_squared() >= 1.0 { continue; }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(0.0, 2.0*PI);
    let z = rng.gen_range(-1.0, 1.0) as f32;
    let r = (1.0 - z * z).sqrt();
    Vec3 {
        e: [
            r*a.cos(),
            r*a.sin(),
            z,
        ]
    }
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if  in_unit_sphere.dot(*normal) > 0.0 { // In the same hemisphere as the normal
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * v.dot(n) * 2.0
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_atat: f32) -> Vec3 {
    let cos_theta = Vec3::dot(&-uv, n);
    let r_out_perp: Vec3 = (uv + n*cos_theta) * etai_over_atat;
    let r_out_parallel: Vec3 = n * -((1.0 - r_out_perp.length_squared()).abs()).sqrt();
    r_out_perp + r_out_parallel
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.x(), -self.y(), -self.z()],
        }  
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

// impl Index<usize> for &Vec3 {
//     type Output = f32;

//     fn index(&self, index: usize) -> &Self::Output {
//         &self.e[index]
//     }
// }

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        let e0 = self.x() + other.x();
        let e1 = self.y() + other.y();
        let e2 = self.z() + other.z();

        *self = Self {
            e: [e0, e1, e2],
        };
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        let e0 = self.x() * rhs;
        let e1 = self.y() * rhs;
        let e2 = self.z() * rhs;

        *self = Self {
            e: [e0, e1, e2],
        };
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        let e0 = self.x() / rhs;
        let e1 = self.y() / rhs;
        let e2 = self.z() / rhs;

        *self = Self {
            e: [e0, e1, e2],
        };
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vec3 {
            e: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ]
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vec3 {
            e: [
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
            ]
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Vec3 {
            e: [
                self.x() * other.x(),
                self.y() * other.y(),
                self.z() * other.z(),
            ]
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [
                self.x() * rhs,
                self.y() * rhs,
                self.z() * rhs,
            ]
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let r = 1.0 / rhs;

        Vec3 {
            e: [
                self.x() * r,
                self.y() * r,
                self.z() * r,
            ]
        }
    }
}