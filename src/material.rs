use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::Vec3;

use rand::Rng;

pub enum MaterialType {
    Lambertian,
    Metal,
    Dielectric,
}

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        incoming_ray: &Ray,
        rec: &HitRecord,
    ) -> Option<(Color, Ray)>;
}

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }

    fn reflect(&self, incoming_ray_direction: Vec3, rec: &HitRecord) -> Option<Ray> {
        let reflected_direction = incoming_ray_direction.reflect(&rec.normal);
        let reflected_ray = Ray::new(rec.p, reflected_direction);
        return Some(reflected_ray);
    }

    fn refract(
        &self,
        incoming_ray_direction: Vec3,
        rec: &HitRecord,
        refraction_ratio: f64,
    ) -> Option<Ray> {
        let cos_theta = (-incoming_ray_direction.dot(&rec.normal)).min(1.0);

        let refracted_direction_orthogonal =
            refraction_ratio * (incoming_ray_direction + cos_theta * rec.normal);
        let refracted_direction_parallel = -(1.0 - refracted_direction_orthogonal.length_squared())
            .abs()
            .sqrt()
            * rec.normal;

        let refracted_direction = refracted_direction_parallel + refracted_direction_orthogonal;

        let refracted_ray = Ray::new(rec.p, refracted_direction);

        return Some(refracted_ray);
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        _rng: &mut rand::rngs::ThreadRng,
        incoming_ray: &Ray,
        rec: &HitRecord,
    ) -> Option<(Color, Ray)> {
        let color = Color::new(1.0, 1.0, 1.0);
        let refraction_index_outside = 1.0;

        let refraction_ratio = if rec.front_face {
            refraction_index_outside / self.refraction_index
        } else {
            self.refraction_index / refraction_index_outside
        };

        // Determine if the material can refract at this angle
        let unit_direction = incoming_ray.direction.normalize();
        let cos_theta = (-unit_direction.dot(&rec.normal)).min(1.);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let scattered_ray: Option<Ray>;
        // TODO(geoff): Implement Schlick's approximation for reflectance
        if cannot_refract {
            scattered_ray = self.reflect(unit_direction, rec);
        } else {
            scattered_ray = self.refract(unit_direction, rec, refraction_ratio);
        }
        return Some((color, scattered_ray.unwrap()));
        // return scattered_ray.map(|ray| (color, ray));
    }
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
    ) -> Option<(Color, Ray)> {
        let mut scatter_direction: Vec3 = rec.normal + Vec3::random_unit_vector(rng);
        if scatter_direction.close_to(Vec3::zeros()) {
            scatter_direction = rec.normal;
        }
        let scattered_ray = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        return Some((attenuation, scattered_ray));
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
    ) -> Option<(Color, Ray)> {
        let reflected = incoming_ray.direction.reflect(&rec.normal);
        let scattered_ray = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;
        return Some((attenuation, scattered_ray));
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

pub static MATERIAL_GLASS: Dielectric = Dielectric {
    refraction_index: 1.5,
};

pub static MATERIALS: [&'static dyn Material; 6] = [
    // Hand defined
    &MATERIAL_CONCRETE,
    &MATERIAL_GROUND,
    &MATERIAL_COPPER,
    &MATERIAL_SILVER,
    &MATERIAL_RED_PLASTIC,
    &MATERIAL_GLASS,
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
        MaterialType::Dielectric => Box::leak(Box::new(Dielectric {
            refraction_index: rng.gen_range(1.0..2.0),
        })),
    }
}
