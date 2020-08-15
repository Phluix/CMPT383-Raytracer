use crate::vec3::Color;
use crate::ray::Ray;

pub fn write_color(pixel_color: Color) {
    // Write the translated [0,255] value of each color component
    let r = (255.999 * pixel_color.r()) as i32;
    let g = (255.999 * pixel_color.g()) as i32;
    let b = (255.999 * pixel_color.b()) as i32;

    println!("{} {} {}", r, g, b);
}

pub fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction().unit_vector();
    // println!("{}", unit_direction.y());
    let t = 0.5 * (unit_direction.y() + 1.0);
    // linear blend: blendedValue = (1-t) * startValue + t * endValue
    Color::new(1.0, 1.0, 1.0) * (1.0-t) + Color::new(0.5, 0.7, 1.0) * t
}