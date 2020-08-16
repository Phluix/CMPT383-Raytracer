use crate::vec3::{Point3, Vec3, random_in_uint_disk};
use crate::ray::Ray;
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn camera(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f32, aspect_ratio: f32, aperature: f32, focus_dist: f32) -> Self {
        let theta = f32::to_radians(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(&(lookfrom - lookat));
        let u = Vec3::unit_vector(&vup.cross(w));
        let v = Vec3::cross(&w, u);

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w*focus_dist;

        let lens_radius = aperature / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = random_in_uint_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::ray(
            self.origin + offset,
            self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin - offset)
    }
}