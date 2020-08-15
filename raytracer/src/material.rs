use crate::ray::Ray;
use crate::vec3::{Vec3, Color, random_unit_vector, reflect, refract};
use crate::hittable::{HitRecord};

use rand::prelude::*;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f32 },
    Dielectric { ref_idx: f32 },
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
    pub fuzz: f32,
}

impl Metal {
    pub fn metal(albedo: &Color, f: f32) -> Self {
        Metal {
            albedo: *albedo,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}


#[derive(Copy, Clone)]
pub struct Dielectric {
    ref_idx: f32
}

impl Dielectric {
    pub fn dielectric(ref_idx: f32) -> Self {
        Dielectric {
            ref_idx
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
        &Material::Metal { albedo, fuzz } => {
            let reflected = reflect(r_in.direction().unit_vector(), rec.normal);
            *scattered = Ray::ray(rec.hit_point, reflected + random_unit_vector()*fuzz);
            *attenuation = albedo;
            return scattered.direction().dot(rec.normal) > 0.0;
        }
        &Material::Dielectric { ref_idx } => {
            *attenuation = Color::new(1.0, 1.0, 1.0);
            let etai_over_etat = if rec.front_face { 1.0 / ref_idx } else { ref_idx };

            let unit_direction = r_in.direction().unit_vector(); 

            let cos_theta = f32::min(Vec3::dot(&-unit_direction, rec.normal), 1.0);
            let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
            if (etai_over_etat * sin_theta) > 1.0 {
                let reflected = reflect(unit_direction,rec.normal);
                *scattered = Ray::ray(rec.hit_point, reflected);
                return true;
            }

            let reflect_prob = schlick(cos_theta, etai_over_etat);
            let mut rng = rand::thread_rng();
            if rng.gen::<f32>() < reflect_prob {
                let reflected = reflect(unit_direction, rec.normal);
                *scattered = Ray::ray(rec.hit_point, reflected);
                return true;
            }
            

            let refracted = refract(unit_direction, rec.normal, etai_over_etat);
            *scattered  = Ray::ray(rec.hit_point, refracted);
            return true;
        }
    }
}

pub fn schlick(cosine: f32, ref_idx: f32)-> f32 {
    let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
    r0 = r0*r0;
    r0 + (1.0-r0) * (1.0-cosine).powf(5.0)
}