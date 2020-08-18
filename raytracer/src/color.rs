use crate::vec3::Color;
use crate::ray::Ray;

pub fn write_color(pixel_color: Color, samples_per_pixel: i32, output: &mut String) {
    // Write the translated [0,255] value of each color component
    // let r = (255.999 * pixel_color.r()) as i32;
    // let g = (255.999 * pixel_color.g()) as i32;
    // let b = (255.999 * pixel_color.b()) as i32;

    let mut r = pixel_color.r();
    let mut g = pixel_color.g();
    let mut b = pixel_color.b();
    
    // Divide the color by the number of samples
    let scale = 1.0 / samples_per_pixel as f32;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();
    
    

    // println!("{} {} {}",
    //     (256.0 * clamp(r, 0.0, 0.999)) as i32,
    //     (256.0 * clamp(g, 0.0, 0.999)) as i32, 
    //     (256.0 * clamp(b, 0.0, 0.999)) as i32);

    *output = format!("{}\n{} {} {}", output,
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32,
        (256.0 * clamp(b, 0.0, 0.999)) as i32,
    );

    
}

pub fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction().unit_vector();
    // println!("{}", unit_direction.y());
    let t = 0.5 * (unit_direction.y() + 1.0);
    // linear blend: blendedValue = (1-t) * startValue + t * endValue
    Color::new(1.0, 1.0, 1.0) * (1.0-t) + Color::new(0.5, 0.7, 1.0) * t
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min { return min; }
    if x > max { return max; }
    x
}