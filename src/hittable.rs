pub use nalgebra::{Point3, Vector3};
pub use super::ray::Ray;
pub use super::material::Material;

pub struct HitRecord {
    pub location: Point3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64, // todo rename to distance
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn new(ray: &Ray, location: Point3<f64>, normal: Vector3<f64>, t: f64, material: Material) -> Self {
        let front_face = ray.direction.dot(&normal) < 0.0;
        HitRecord { location, normal: if front_face { normal } else { -normal }, t, front_face, material }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub enum Object {
    Sphere { material: Material, center: Point3<f64>, radius: f64 }
}

impl Hittable for Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Object::Sphere { material, center, radius } => {
                let oc = &ray.origin - center;
                let a = ray.direction.norm_squared();
                let half_b = oc.dot(&ray.direction);
                let c = oc.norm_squared() - radius * radius;
                let discriminant = half_b * half_b - a * c;

                if discriminant > 0.0 {
                    let disc_sqrt = discriminant.sqrt();

                    for disc in [-disc_sqrt, disc_sqrt].iter() {
                        let root = (-half_b + disc) / a;
                        if t_min < root && root < t_max {
                            let pos = ray.at(root);
                            return Some(HitRecord::new(&ray, pos, (pos - center) / *radius, root, *material));
                        }
                    }
                }

                None
            }
        }
    }
}
