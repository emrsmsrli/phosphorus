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

fn refract(uv: &Vector3<f64>, normal: &Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let cos_theta = (-uv).dot(normal);
    let r_out_parallel = (uv + cos_theta * normal) * etai_over_etat;
    let r_out_perp = -(1.0 - r_out_parallel.norm_squared()).sqrt() * normal;
    r_out_parallel + r_out_perp
}

fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - n * 2.0 * v.dot(n)
}

fn schlick(cos: f64, dielectric_constant: f64) -> f64 {
    let mut r0 = (1.0 - dielectric_constant) / (1.0 + dielectric_constant);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
}

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Color),
    Metal(Color, f64 /*fuzz*/),
    Dielectric(f64 /*dielectric_constant*/),
}

impl Material {
    pub fn scatter(&self, rng: &mut impl Rng, ray: &Ray, hit_record: &HitRecord) -> (Option<Ray>, Option<Color>) {
        match self {
            Material::Lambertian(color) => {
                let direction = hit_record.normal + random_in_hemisphere(rng, &hit_record.normal);
                (Some(Ray::new(hit_record.location, direction)), Some(color.clone()))
            }
            Material::Metal(color, fuzz) => {
                let reflected = reflect(&ray.direction.normalize(), &hit_record.normal);
                if reflected.dot(&hit_record.normal) > 0.0 {
                    return (
                        Some(Ray::new(hit_record.location, reflected + random_vector_in_unit_sphere(rng) * *fuzz)),
                        Some(color.clone())
                    );
                }

                (None, None)
            }
            Material::Dielectric(dielectric_constant) => {
                let etai_over_etat = if hit_record.front_face { 1.0 / *dielectric_constant } else { *dielectric_constant };
                let unit = ray.direction.normalize();

                let x = (-unit).dot(&hit_record.normal);
                let cos_theta = if x < 1.0 { x } else { 1.0 };
                let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
                if etai_over_etat * sin_theta > 1.0 {
                    let reflected = reflect(&unit, &hit_record.normal);
                    return (Some(Ray::new(hit_record.location, reflected)), Some(Color::new(1.0, 1.0, 1.0)));
                }

                let reflect_prob = schlick(cos_theta, etai_over_etat);
                if rng.gen_range(0.0, 1.0) < reflect_prob {
                    let reflected = reflect(&unit, &hit_record.normal);
                    return (Some(Ray::new(hit_record.location, reflected)), Some(Color::new(1.0, 1.0, 1.0)));
                }

                let refracted = refract(&unit, &hit_record.normal, etai_over_etat);
                (Some(Ray::new(hit_record.location, refracted)), Some(Color::new(1.0, 1.0, 1.0)))
            }
        }
    }
}
