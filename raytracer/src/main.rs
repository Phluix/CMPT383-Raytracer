#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod vec3;
mod ray;
mod color;
mod hittable;
mod sphere;
mod hittable_list;

use ray::Ray;
use vec3::{Vec3, Point3, Color};
use hittable::{Hittable, HitRecord};
use hittable_list::HittableList;
use sphere::Sphere;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
const MAX_VALUE: i32 = 255;
const INFINITY: f32 = f32::INFINITY;


fn main() {
    
    // World
    let mut world = HittableList { objects: Vec::new() };
    world.add(Box::new(Sphere {
        center: Point3::new(0.5, 0.0, -1.0),
        radius: 0.5,
    }));
    world.add(Box::new(Sphere {
        center: Point3::new(-0.5, 0.0, -1.0),
        radius: 0.5,
    }));


    write_ppm(IMAGE_WIDTH, IMAGE_HEIGHT, MAX_VALUE, &world);
}

fn write_ppm<T: Hittable>(w: i32, h: i32, max: i32, world: &T) {

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0,0.0,0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);



    println!("P3\n{} {}\n{}\n", w, h, max);

    for j in (0..(h-1)).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..w {
            // let r = i as f32 / w as f32;
            // let g = j as f32 / h as f32;
            // let b = 0.25 as f32;

            // let ir = (255.99 * r) as i32;
            // let ig = (255.99 * g) as i32;
            // let ib = (255.99 * b) as i32;
            let u = i as f32 / (w-1) as f32;
            let v = j as f32 / (h-1) as f32;
            let r: Ray = Ray::ray(origin, lower_left_corner + horizontal * u + vertical * v - origin);
            let pixel_color: Color = ray_color(&r, world);
            color::write_color(pixel_color);
        }
    }
}

fn hit_sphere(sphere_center: Point3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin() - sphere_center;
    let a = r.direction().length_squared();
    let half_b = oc.dot(r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b*half_b - a*c;

    if discriminant < 0.0 {
        return -1.0
    } else {
        return (-half_b -discriminant.sqrt()) / a
    }
}

fn ray_color<T: Hittable>(r: &Ray, world: &T) -> Color {
    if let Some(rec) = world.hit(r, 0.0, INFINITY) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }



    // let h = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    // if h > 0.0 {
    //     let n = r.at(h) - Vec3::new(0.0, 0.0, -1.0);
    //     let u_n = n.unit_vector();
    //     return Color::new(u_n.x()+1.0, u_n.y()+1.0, u_n.z()+1.0) * 0.5
    // }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    // linear blend: blendedValue = (1-t) * startValue + t * endValue
    Color::new(1.0, 1.0, 1.0) * (1.0-t) + Color::new(0.5, 0.7, 1.0) * t
}