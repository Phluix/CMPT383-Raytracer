#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod vec3;
mod ray;
mod color;

use ray::Ray;
use vec3::{Vec3, Point3, Color};

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
const MAX_VALUE: i32 = 255;


fn main() {
    // // IMage
    // let aspect_ratio = 16.0 / 9.0;
    // let image_width = 400;
    // let image_height = (image_width as f32 / aspect_ratio) as i32;

    // // Camera
    // let viewport_height = 2.0;
    // let viewport_width = aspect_ratio * viewport_height;
    // let focal_length = 1.0;

    // let origin = Point3::new(0.0,0.0,0.0);
    // let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    // let vertical = Vec3::new(0.0, viewport_height, 0.0);
    // let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);
    
    write_ppm(IMAGE_WIDTH, IMAGE_HEIGHT, MAX_VALUE);
}

fn write_ppm(w: i32, h: i32, max: i32) {

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
            let pixel_color: Color = ray_color(&r);
            color::write_color(pixel_color);
        }
    }
}

fn hit_sphere(sphere_center: Point3, radius: f32, r: &Ray) -> bool {
    let oc = r.origin() - sphere_center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;

    discriminant > 0.0
}

fn ray_color(r: &Ray) -> Color {
    if (hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r)) {
        return Color::new(1.0, 0.0, 0.0)
    }
    let unit_direction = r.direction().unit_vector();
    // println!("{}", unit_direction.y());
    let t = 0.5 * (unit_direction.y() + 1.0);
    // linear blend: blendedValue = (1-t) * startValue + t * endValue
    Color::new(1.0, 1.0, 1.0) * (1.0-t) + Color::new(0.5, 0.7, 1.0) * t
}