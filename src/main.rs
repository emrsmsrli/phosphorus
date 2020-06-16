mod ppm;
mod ray;

use std::path::Path;
use ray::Ray;
use nalgebra::{Point3, Vector3};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const W: usize = 384;
const H: usize = (W as f64 / ASPECT_RATIO) as usize;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

fn ray_color(r: &Ray) -> ppm::Color {
    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction[1] + 1.0);
    return ppm::Color::new(1.0, 1.0, 1.0) * (1.0 - t)
        + ppm::Color::new(0.5, 0.7, 1.0) * t;
}

fn main() {
    let mut ppm = ppm::Writer::new(W, H);

    let origin = Point3::origin();
    let horizontal = Vector3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vector3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, FOCAL_LENGTH);

    for y in (0..H).rev() {
        for x in 0..W {
            let u = x as f64 / (W - 1) as f64;
            let v = y as f64 / (H - 1) as f64;

            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            ppm[(x, H - y - 1)] = ray_color(&ray);
        }
    }

    ppm.save_to_file(Path::new(r"C:\Users\Emre\Desktop\a.ppm"))
}
