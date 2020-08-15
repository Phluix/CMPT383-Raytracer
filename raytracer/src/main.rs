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

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
const MAX_VALUE: i32 = 255;
const INFINITY: f32 = f32::INFINITY;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;


fn main() {
    // World
    let mut world = HittableList { objects: Vec::new() };

    let material_ground =  Color::new(0.3, 0.3, 0.3);
    let material_center =  Color::new(0.1, 0.2, 0.5);
    // let material_left = Color::new(0.8, 0.8, 0.8);
    let material_right = Color::new(0.8, 0.6, 0.2);

    world.add(Box::new(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Material::Lambertian{ albedo: material_ground },
    }));
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Lambertian{ albedo: material_center },
    }));
    world.add(Box::new(Sphere {
        center: Point3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Dielectric{ ref_idx: 1.5 },
    }));
    world.add(Box::new(Sphere {
        center: Point3::new(-1.0, 0.0, -1.0),
        radius: -0.4,
        material: Material::Dielectric{ ref_idx: 1.5 },
    }));
    world.add(Box::new(Sphere {
        center: Point3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Metal{ albedo: material_right, fuzz: 1.0 },
    }));



    write_ppm(IMAGE_WIDTH, IMAGE_HEIGHT, MAX_VALUE, &world);
}

fn write_ppm<T: Hittable>(w: i32, h: i32, max: i32, world: &T) {
    let cam = Camera::camera();
    let mut rng = rand::thread_rng();

    println!("P3\n{} {}\n{}\n", w, h, max);

    for j in (0..(h-1)).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..w {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + rng.gen::<f32>()) / (w-1) as f32;
                let v = (j as f32 + rng.gen::<f32>()) / (h-1) as f32;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, world, MAX_DEPTH);
            }
            color::write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}

fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }


    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        // // let target = rec.hit_point +  rec.normal + vec3::random_in_unit_sphere();
        // let target = rec.hit_point +  rec.normal + vec3::random_unit_vector();
        // // let target = rec.hit_point +  rec.normal + vec3::random_in_hemisphere(&rec.normal);

        // return ray_color(&Ray::ray(rec.hit_point, target - rec.hit_point), world, depth-1) * 0.5;
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
    Color::new(1.0, 1.0, 1.0) * (1.0-t) + Color::new(0.5, 0.7, 1.0) * t
}