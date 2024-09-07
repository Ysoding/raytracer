use crate::{HitRecord, Ray, ScatterRecord};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
}
