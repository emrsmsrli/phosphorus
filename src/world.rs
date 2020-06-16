use crate::hittable::{Object, Hittable, HitRecord};
use crate::ray::Ray;

pub struct World {
    objects: Vec<Object>
}

impl World {
    pub fn new() -> Self {
        World { objects: vec![] }
    }

    pub fn add(&mut self, obj: Object) {
        self.objects.push(obj)
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut latest_hit_record: Option<HitRecord> = None;

        for object in self.objects.iter() {
            if let Some(hit_record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                latest_hit_record = Some(hit_record);
            }
        }

        latest_hit_record
    }
}
