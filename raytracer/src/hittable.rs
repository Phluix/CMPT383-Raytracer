use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};
use crate::material::Material;

pub struct HitRecord{
    pub t: f32,
    pub hit_point: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Material,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(*outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}