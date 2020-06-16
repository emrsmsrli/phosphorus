use nalgebra::{Point3, Vector3};
use crate::ray::Ray;

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

pub struct Camera {
    position: Point3<f64>,
    lower_left_corner: Point3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
}

impl Camera {
    pub fn new() -> Self {
        let position = Point3::origin();
        let horizontal = Vector3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vector3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        Camera {
            position,
            lower_left_corner: position - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, FOCAL_LENGTH),
            horizontal,
            vertical
        }
    }

    pub fn new_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.position, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.position)
    }
}
