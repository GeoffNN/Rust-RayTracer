use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::Vec3;

use rand::Rng;

pub enum MaterialType {
    Lambertian,
    Metal,
}

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        rng: &mut rand::rngs::ThreadRng,
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
        rng: &mut rand::rngs::ThreadRng,
        _incoming_ray: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered_ray: &mut Ray,
    ) -> bool {
        let mut scatter_direction: Vec3 = rec.normal + Vec3::random_unit_vector(rng);
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
        mut _rng: &mut rand::rngs::ThreadRng,
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

pub static MATERIAL_CONCRETE: Lambertian = Lambertian {
    albedo: Color::new_const(0.5, 0.5, 0.5),
};
pub static MATERIAL_GROUND: Lambertian = Lambertian {
    albedo: Color::new_const(0.8, 0.8, 0.),
};
pub static MATERIAL_COPPER: Metal = Metal {
    albedo: Color::new_const(0.7, 0.5, 0.3),
};
pub static MATERIAL_SILVER: Metal = Metal {
    albedo: Color::new_const(0.9, 0.9, 0.9),
};
pub static MATERIAL_RED_PLASTIC: Lambertian = Lambertian {
    albedo: Color::new_const(0.9, 0.1, 0.1),
};

pub static MATERIALS: [&'static dyn Material; 5] = [
    // Hand defined
    &MATERIAL_CONCRETE,
    &MATERIAL_GROUND,
    &MATERIAL_COPPER,
    &MATERIAL_SILVER,
    &MATERIAL_RED_PLASTIC,
];

pub fn random_material_from_presets(rng: &mut rand::rngs::ThreadRng) -> &'static dyn Material {
    let material_index = rng.gen_range(0..MATERIALS.len());
    MATERIALS[material_index]
}

fn random_material_type(rng: &mut rand::rngs::ThreadRng) -> MaterialType {
    let material_type_index = rng.gen_range(0..2);
    match material_type_index {
        0 => MaterialType::Lambertian,
        1 => MaterialType::Metal,
        _ => panic!("Invalid material type index"),
    }
}

pub fn random_static_material(rng: &mut rand::rngs::ThreadRng) -> &'static dyn Material {
    let material_type = random_material_type(rng);
    match material_type {
        MaterialType::Lambertian => Box::leak(Box::new(Lambertian {
            albedo: Color::random(rng),
        })),
        MaterialType::Metal => Box::leak(Box::new(Metal {
            albedo: Color::random(rng),
        })),
    }
}
