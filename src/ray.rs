pub use nalgebra::{Vector3, Point3};

pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>
}

impl Ray {
    #[allow(dead_code)]
    pub fn at(&self, t: f64) -> Point3<f64> {
        &self.origin + &self.direction * t
    }

    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Ray {
        Ray { origin, direction }
    }
}
