mod ppm;
mod ray;
mod hittable;
mod world;

use std::path::Path;
use nalgebra::{Point3, Vector3};
use crate::ray::Ray;
use crate::hittable::Object::Sphere;
use crate::hittable::Hittable;
use crate::world::World;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const W: usize = 384;
const H: usize = (W as f64 / ASPECT_RATIO) as usize;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

fn ray_color(ray: &Ray, world: &World) -> ppm::Color {
    if let Some(hit_record) = world.hit(ray, 0.0, f64::INFINITY) {
        return ppm::Color::new(
            hit_record.normal[0] + 1.0,
            hit_record.normal[1] + 1.0,
            hit_record.normal[2] + 1.0) * 0.5;
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction[1] + 1.0);
    ppm::Color::new(1.0, 1.0, 1.0) * (1.0 - t)
        + ppm::Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let mut ppm = ppm::Writer::new(W, H);
    let mut world = World::new();

    world.add(Sphere { center: Point3::new(0.0, 0.0, -1.0), radius: 0.5 });
    world.add(Sphere { center: Point3::new(0.0, -100.5, -1.0), radius: 100.0 });

    let origin = Point3::origin();
    let horizontal = Vector3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vector3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, FOCAL_LENGTH);

    for y in (0..H).rev() {
        for x in 0..W {
            let u = x as f64 / (W - 1) as f64;
            let v = y as f64 / (H - 1) as f64;

            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            ppm[(x, H - y - 1)] = ray_color(&ray, &world);
        }
    }

    ppm.save_to_file(Path::new(r"C:\Users\Emre\Desktop\a.ppm"))
}
