mod ppm;
mod ray;
mod hittable;
mod world;
mod camera;
mod material;

use std::io::Write;
use std::path::Path;
use nalgebra::{Point3, Vector3};
use rand::Rng;

use crate::ray::Ray;
use crate::hittable::Object::Sphere;
use crate::hittable::Hittable;
use crate::world::World;
use crate::camera::{ASPECT_RATIO, Camera};
use crate::ppm::Color;
use crate::material::Material;

const W: usize = 720;
const H: usize = (W as f64 / ASPECT_RATIO) as usize;

fn ray_color(rng: &mut impl Rng, ray: &Ray, world: &World, depth: i32) -> ppm::Color {
    if depth <= 0 {
        return ppm::Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = world.hit(ray, 0.001, std::f64::INFINITY) {
        if let (Some(reflected), color) = hit_record.material.scatter(rng, ray, &hit_record) {
            return color.unwrap() * ray_color(rng, &reflected, world, depth - 1);
        }

        return Color::new(0.0, 0.0, 0.0);
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
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);

    let camera = Camera::new(
        look_from.clone(),
        look_at.clone(),
        Vector3::new(0.0, 1.0, 0.0),
        20.0, ASPECT_RATIO, 0.1, 10.0);
    let mut ppm = ppm::Writer::new(W, H);
    let mut world = World::new();
    let mut rand = rand::thread_rng();

    world.add(Sphere {
        center: Point3::new(0.0, -1000.0, -1.0),
        radius: 1000.0,
        material: Material::Lambertian(Color::new(0.5, 0.5, 0.5)),
    });

    for a in -11..=11 {
        for b in -11..=11 {
            let mat_prob = rand.gen_range(0.0, 1.0);
            let center = Point3::new(
                a as f64 + 0.9 * rand.gen_range(0.0, 1.0),
                0.2,
                b as f64 + 0.9 * rand.gen_range(0.0, 1.0));

            if (&center - Point3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                if mat_prob < 0.8 {
                    world.add(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Lambertian(Color::new(rand.gen_range(0.0, 1.0), rand.gen_range(0.0, 1.0), rand.gen_range(0.0, 1.0))),
                    });
                } else if mat_prob < 0.95 {
                    world.add(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Metal(Color::new(
                            rand.gen_range(0.5, 1.0), rand.gen_range(0.5, 1.0), rand.gen_range(0.5, 1.0)), rand.gen_range(0.0, 0.5)),
                    });
                } else {
                    world.add(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Dielectric(1.5),
                    });
                }
            }
        }
    }

    world.add(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Dielectric(1.5),
    });
    world.add(Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Lambertian(Color::new(0.4, 0.2, 0.1)),
    });
    world.add(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal(Color::new(0.7, 0.6, 0.5), 0.0),
    });

    for y in (0..H).rev() {
        println!("generating px y {}", H - 1 - y);
        std::io::stdout().flush().ok();
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
