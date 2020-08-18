#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod vec3;
mod ray;
mod color;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;
mod material;

use ray::Ray;
use vec3::{Vec3, Point3, Color};
use hittable::{Hittable, HitRecord};
use hittable_list::HittableList;
use sphere::Sphere;
use camera::Camera;
use material::{Material, Lambertian, Metal, scatter};

use rand::prelude::*;
use std::f32::consts::PI;
use rayon::prelude::*;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
const MAX_VALUE: i32 = 255;
const INFINITY: f32 = f32::INFINITY;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 12;

pub struct Scene {
    pub ground: Vec<f32>,
    pub skycolor: Vec<f32>,
    pub lookfrom: Vec<f32>,
    pub lookat: Vec<f32>,
    pub vup: Vec<f32>,
    pub sphere1_center: Vec<f32>,
    pub sphere1_radius: f32,
    pub sphere2_center: Vec<f32>,
    pub sphere2_color: Vec<f32>,
    pub sphere2_radius: f32,
    pub sphere3_center: Vec<f32>,
    pub sphere3_color: Vec<f32>,
    pub sphere3_radius: f32,
}

pub fn render(s: Scene) -> String {
    // World
    let world = scene();

    return write_ppm(IMAGE_WIDTH, IMAGE_HEIGHT, MAX_VALUE, &world);
}

pub fn test() {
    
}


fn scene() -> HittableList {
    // Collection of spheres
    let mut world = HittableList { objects: Vec::new() };

    // Color of ground (ground is essentially an enlarged sphere)
    let ground_color = Color::new(0.5, 0.5, 0.5);
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::Lambertian{ albedo: ground_color },
    }));

    // Sphere 1
    let material1 = Material::Dielectric{ ref_idx: 1.5 };
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1,
    }));

    // Sphere 2
    let material2 = Material::Lambertian{ albedo: Color::new(0.4, 0.2, 0.1)};
    world.add(Box::new(Sphere {
        center: Point3::new(-1.8, 1.0, 0.0),
        radius: 1.0,
        material: material2,
    }));

    // Sphere 3
    let material3 = Material::Metal{ albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0 };
    world.add(Box::new(Sphere {
        center: Point3::new(1.8, 1.0, 0.0),
        radius: 1.0,
        material: material3,
    }));

    world
}

fn write_ppm<T: Hittable>(w: i32, h: i32, max: i32, world: &T) -> String {
    let lookfrom = Point3::new(5.0, 10.0, 10.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    // let dist_to_focus = (lookfrom-lookat).length();
    let dist_to_focus = 10.0;
    let aperature = 0.0;

    let cam = Camera::camera(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperature,
        dist_to_focus,
    );
    let mut rng = rand::thread_rng();

    let mut output = format!("P3\n{} {}\n{}\n", w, h, max);

    for j in (0..(h)).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..w {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + rng.gen::<f32>()) / (w-1) as f32;
                let v = (j as f32 + rng.gen::<f32>()) / (h-1) as f32;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, world, MAX_DEPTH);
            }
            color::write_color(pixel_color, SAMPLES_PER_PIXEL, &mut output);
        }
    }

    output
}

fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        let mut scattered: Ray = Default::default();
        let mut attenuation: Color = Default::default();
        if scatter(&rec.material, r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth-1);
        }
        return Color::new(0.0, 0.0, 0.0)
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    // linear blend: blendedValue = (1-t) * startValue + t * endValue
    Color::new(1.0, 1.0, 1.0) * (1.0-t) + Color::new(0.7, 0.5, 1.8) * t
}