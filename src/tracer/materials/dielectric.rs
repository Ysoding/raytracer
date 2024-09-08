use crate::{random_f64_range, reflect, refract, Material, Ray, ScatterRecord, Vec3};

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &crate::Ray,
        hit_record: &crate::HitRecord,
    ) -> Option<crate::ScatterRecord> {
        let ri = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let uint_direction = ray.direction.unit();
        let cos_theta = (-uint_direction).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction =
            if cannot_refract || self.reflectance(cos_theta, ri) > random_f64_range(0.0, 1.0) {
                reflect(uint_direction, hit_record.normal)
            } else {
                refract(uint_direction, hit_record.normal, ri)
            };

        Some(ScatterRecord {
            attenuation: Vec3::ones(),
            scatter_ray: Ray::new(hit_record.p, direction),
        })
    }
}
