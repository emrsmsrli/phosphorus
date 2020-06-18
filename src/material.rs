use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::ppm::Color;
use nalgebra::Vector3;
use rand::Rng;

fn random_vector_in_unit_sphere(rng: &mut impl Rng) -> Vector3<f64> {
    let a = rng.gen_range(0.0, 2.0 * std::f64::consts::PI);
    let z = rng.gen_range(-1.0f64, 1.0);
    let r = (1.0 - z * z).sqrt();
    Vector3::new(r * a.cos(), r * a.sin(), z)
}

fn random_in_hemisphere(rng: &mut impl Rng, normal: &Vector3<f64>) -> Vector3<f64> {
    let in_unit_sphere = random_vector_in_unit_sphere(rng);
    if in_unit_sphere.dot(normal) > 0.0 { // In the same hemisphere as the normal
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Color),
    Metal(Color, f64 /*fuzz*/),
}

impl Material {
    pub fn scatter(&self, rng: &mut impl Rng, ray: &Ray, hit_record: &HitRecord) -> (Option<Ray>, Option<Color>) {
        match self {
            Material::Lambertian(color) => {
                let direction = hit_record.normal + random_in_hemisphere(rng, &hit_record.normal);
                (Some(Ray::new(hit_record.location, direction)), Some(color.clone()))
            },
            Material::Metal(color, fuzz) => {
                let reflect = |v: &Vector3<f64>, n: &Vector3<f64>| { v - n * 2.0 * v.dot(n) };
                let reflected = reflect(&ray.direction.normalize(), &hit_record.normal);
                if reflected.dot(&hit_record.normal) > 0.0 {
                    return (
                        Some(Ray::new(hit_record.location, reflected + random_vector_in_unit_sphere(rng) * *fuzz)),
                        Some(color.clone())
                    );
                }

                (None, None)
            }
        }
    }
}
