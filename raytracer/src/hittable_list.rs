use crate::hittable::{Hittable, HitRecord};
use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;
use crate::sphere::Sphere;

pub struct HittableList {
    /* https://stackoverflow.com/questions/25818082/vector-of-objects-belonging-to-a-trait
    Shows how to have a vector of objects that all have a similar trait. 
    Traits by default don't have a size, so we use box which creates a "Fat pointer" if a trait */
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn hittable_list(&mut self, object: Box<dyn Hittable>) -> HittableList {
        HittableList {
            objects: vec![object],
        }        
    }

    pub fn clear(&mut self) {
        self.objects = Vec::new();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        // dereferencing Box<T> returns T
        self.objects.push(object); 
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for obj in  &self.objects {
            if let Some(rec) = obj.hit(r, t_min, closest_so_far) {
                closest_so_far =  rec.t;
                hit_record = Some(rec);
            }
        }
        hit_record
    }
}