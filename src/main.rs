mod ppm;
mod ray;
mod hittable;
mod world;
mod camera;

use std::path::Path;
use nalgebra::{Point3, Vector3};
use rand::Rng;

use crate::ray::Ray;
use crate::hittable::Object::Sphere;
use crate::hittable::Hittable;
use crate::world::World;
use crate::camera::{ASPECT_RATIO, Camera};
use crate::ppm::Color;

const W: usize = 384;
const H: usize = (W as f64 / ASPECT_RATIO) as usize;

fn random_vector_in_unit_sphere(rng: &mut impl Rng) -> Vector3<f64> {
    let a = rng.gen_range(0.0, 2.0 * std::f64::consts::PI);
    let z = rng.gen_range(-1.0f64, 1.0);
    let r = (1.0 - z * z).sqrt();
    Vector3::new(r * a.cos(), r * a.sin(), z)
}

fn ray_color(rng: &mut impl Rng, ray: &Ray, world: &World, depth: i32) -> ppm::Color {
    if depth <= 0 {
        return ppm::Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = world.hit(ray, 0.001, f64::INFINITY) {
        let target = hit_record.location + random_vector_in_unit_sphere(rng) + hit_record.normal;
        let new_ray = Ray::new(hit_record.location, target - hit_record.location);
        return ray_color(rng, &new_ray, world, depth - 1) * 0.5;
    }

    // background
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction[1] + 1.0);
    ppm::Color::new(1.0, 1.0, 1.0) * (1.0 - t)
        + ppm::Color::new(0.5, 0.7, 1.0) * t
}

const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: i32 = 100;

fn main() {
    let camera = Camera::new();
    let mut ppm = ppm::Writer::new(W, H);
    let mut world = World::new();
    let mut rand = rand::thread_rng();

    world.add(Sphere { center: Point3::new(0.0, 0.0, -1.0), radius: 0.5 });
    world.add(Sphere { center: Point3::new(0.0, -100.5, -1.0), radius: 100.0 });

    for y in (0..H).rev() {
        for x in 0..W {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (x as f64 + rand.gen_range(0.0, 1.0)) / (W - 1) as f64;
                let v = (y as f64 + rand.gen_range(0.0, 1.0)) / (H - 1) as f64;
                let ray = camera.new_ray(u, v);
                color = color + ray_color(&mut rand, &ray, &world, MAX_DEPTH);
            }

            // gamma correction
            const GAMMA_SCALE: f64 = 1.0 / SAMPLES_PER_PIXEL as f64;
            ppm[(x, H - y - 1)] = ppm::Color::new(
                (color.r * GAMMA_SCALE).sqrt(),
                (color.g * GAMMA_SCALE).sqrt(),
                (color.b * GAMMA_SCALE).sqrt(),
            );
        }
    }

    ppm.save_to_file(Path::new(r"C:\Users\Emre\Desktop\a.ppm"))
}
