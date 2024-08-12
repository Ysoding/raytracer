use crate::tracer::{hit_record::HitRecord, hittable::Hittable, ray::Ray, vec3::Vec3};

pub struct SphereRenderer {
    center: Vec3,
    radius: f64,
}

impl Hittable for SphereRenderer {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squard();
        let h = ray.direction.dot(oc);
        let c = oc.length_squard() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if root <= t_min || t_max <= root {
            root = (h + sqrtd) / a;
            if root <= t_min || t_max <= root {
                return None;
            }
        }

        let p = ray.at(root);
        let normal = (p - self.center) / self.radius;
        let t = root;

        Some(HitRecord { normal, p, t })
    }
}
