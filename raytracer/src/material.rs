use crate::ray::Ray;
use crate::vec3::{Color, random_unit_vector, reflect};
use crate::hittable::{HitRecord};

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color },
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn lambertian(albedo: &Color) -> Self {
        Lambertian {
            albedo: *albedo,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub fn metal(albedo: &Color) -> Self {
        Metal {
            albedo: *albedo,
        }
    }
}

pub fn scatter(material: &Material, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
    match material {
        &Material::Lambertian { albedo } => {
            let scatter_direction = rec.normal + random_unit_vector();
            *scattered = Ray::ray(rec.hit_point, scatter_direction);
            *attenuation = albedo;
            return true;
        }
        &Material::Metal { albedo } => {
            let reflected = reflect(r_in.direction().unit_vector(), rec.normal);
            *scattered = Ray::ray(rec.hit_point, reflected);
            *attenuation = albedo;
            return scattered.direction().dot(rec.normal) > 0.0;
        }
    }
}