use nalgebra::{Point3, Vector3};
use crate::ray::Ray;
use rand::Rng;

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;

pub struct Camera {
    position: Point3<f64>,
    lower_left_corner: Point3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Point3<f64>,
        look_at: Point3<f64>,
        vup: Vector3<f64>,
        vfov: f64,
        aspect_r: f64,
        aperture: f64,
        focus_distance: f64) -> Self {

        let theta = vfov.to_radians();
        let h = (theta * 0.5).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_r;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let horizontal = focus_distance * u * viewport_width;
        let vertical = focus_distance * v * viewport_height;

        Camera {
            position: look_from,
            lower_left_corner: look_from - horizontal / 2.0 - vertical / 2.0 - focus_distance * w,
            horizontal,
            vertical,
            u, v,
            lens_radius: aperture / 2.0
        }
    }

    fn random_in_unit_disk(&self) -> Vector3<f64> {
        let mut r = rand::thread_rng();
        loop {
            let p = Vector3::new(r.gen_range(-1.0, 1.0), r.gen_range(-1.0, 1.0), 0.0);
            if p.norm_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn new_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * self.random_in_unit_disk();
        let offset = &self.u * rd.x + &self.v * rd.y;
        Ray::new(
            &self.position + offset,
            &self.lower_left_corner + u * &self.horizontal + v * &self.vertical - &self.position - offset
        )
    }
}
