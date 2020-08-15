use crate::vec3::{Point3};
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};
use crate::material::Material;

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere{
    pub fn sphere(center: Point3, radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>{
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminat = half_b*half_b - a*c;

        if discriminat > 0.0 {
            let root = discriminat.sqrt();

            let mut temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let outward_normal = (r.at(temp) - self.center) / self.radius;
                let mut h = HitRecord{
                    t: temp,
                    hit_point: r.at(temp),
                    normal: Default::default(),
                    front_face: Default::default(),
                    material: self.material,
                };
                h.set_face_normal(r, &outward_normal);
                return Some(h);
            }

            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let outward_normal = (r.at(temp) - self.center) / self.radius;
                let mut h = HitRecord{
                    t: temp,
                    hit_point: r.at(temp),
                    normal: Default::default(),
                    front_face: Default::default(),
                    material: self.material,
                };
                h.set_face_normal(r, &outward_normal);
                return Some(h);
            }
        }
        None
    }
}