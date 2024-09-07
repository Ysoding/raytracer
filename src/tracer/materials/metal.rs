use crate::{random_uint_vector, reflect, HitRecord, Material, Ray, ScatterRecord, Vec3};

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let mut reflected = reflect(ray.direction, hit_record.normal);
        reflected = reflected.unit() + (self.fuzz * random_uint_vector());

        let scatter_ray = Ray::new(hit_record.p, reflected);
        if scatter_ray.direction.dot(hit_record.normal) > 0.0 {
            return Some(ScatterRecord {
                attenuation: self.albedo,
                scatter_ray,
            });
        }
        None
    }
}
