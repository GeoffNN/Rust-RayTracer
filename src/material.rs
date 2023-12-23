use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::Vec3;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        incoming_ray: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered_ray: &mut Ray,
    ) -> bool;
}

#[derive(Default)]
pub struct Lambertian {
    albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _incoming_ray: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered_ray: &mut Ray,
    ) -> bool {
        let mut scatter_direction: Vec3 = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.close_to(Vec3::zeros()) {
            scatter_direction = rec.normal;
        }
        *scattered_ray = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}

#[derive(Default)]
pub struct Metal {
    albedo: Color,
    //TODO(geoff): add fuzziness
}

impl Material for Metal {
    fn scatter(
        &self,
        incoming_ray: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered_ray: &mut Ray,
    ) -> bool {
        let reflected = incoming_ray.direction.reflect(&rec.normal);
        *scattered_ray = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        return true;
    }
}

// TODO(geoff): Add dielectric material

pub const MATERIAL_CONCRETE: Lambertian = Lambertian {
    albedo: Color::new_const(0.5, 0.5, 0.5),
};

pub const MATERIAL_GROUND: Lambertian = Lambertian {
    albedo: Color::new_const(0.8, 0.8, 0.),
};
pub const MATERIAL_COPPER: Metal = Metal {
    albedo: Color::new_const(0.7, 0.5, 0.3),
};
pub const MATERIAL_SILVER: Metal = Metal {
    albedo: Color::new_const(0.9, 0.9, 0.9),
};
pub const MATERIAL_RED_PLASTIC: Lambertian = Lambertian {
    albedo: Color::new_const(0.9, 0.1, 0.1),
};
