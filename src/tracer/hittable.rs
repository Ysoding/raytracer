use crate::Interval;

use super::{hit_record::HitRecord, ray::Ray};

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut res: Option<HitRecord> = None;
        for object in &self.objects {
            if let Some(tmp) = object.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = tmp.t;
                res = Some(tmp);
            }
        }
        res
    }
}
