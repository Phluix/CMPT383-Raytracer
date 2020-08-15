use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f32,
}

pub trait Hittable {
    fn hit(r: &Ray, t_min: f32, t_max: f32, rec: &HitRecord) {

    }
}