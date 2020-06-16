pub use nalgebra::{Point3};
pub use super::ray::Ray;

pub enum Hittable {
    Sphere { center: Point3<f64>, radius: f64 }
}

impl Hittable {
    pub fn hit(&self, ray: &Ray) -> bool {
        match &self {
            // b2 - 4ac > 0
            Hittable::Sphere { center, radius } => {
                let oc = &ray.origin - center;
                let a = ray.direction.dot(&ray.direction);
                let b = 2.0 * oc.dot(&ray.direction);
                let c = oc.dot(&oc) - radius * radius;
                let discriminant = b * b - a * c * 4.0;
                discriminant > 0.0
            }
        }
    }
}
