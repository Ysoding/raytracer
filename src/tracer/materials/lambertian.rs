use crate::{random_uint_vector, HitRecord, Material, Ray, ScatterRecord, Vec3};

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let mut direction = hit_record.normal + random_uint_vector();
        if direction.near_zero() {
            direction = hit_record.normal;
        }
        Some(ScatterRecord {
            attenuation: self.albedo,
            scatter_ray: Ray::new(hit_record.p, direction - hit_record.p),
        })
    }
}
