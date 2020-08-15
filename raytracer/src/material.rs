use crate::ray::Ray;
use crate::vec3::{Color};

pub struct Material {
}

pub struct HitRecord {
}

impl Material {
    pub fn scatter(r_in: &Ray, hit_record: &HitRecord, attenuation: &Color, scattered: &Ray) {
    }
}