use crate::{
    color::Color,
    geometry::{
        ray::Ray,
        vec3::{self},
    },
    hittable::hittable::HitRecord,
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color, // to represent gradual loss of ray strength
        scattered: &mut Ray,
    ) -> bool;
}

// struct to model light scatter
pub struct Lambertian {
    albedo: Color, // latin for whiteness, used to define fractional reflectance
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal_surface + vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal_surface;
        }
        *attenuation = self.albedo;
        *scattered = Ray::new(rec.p, scatter_direction);
        true
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = vec3::reflect(vec3::unit_vector(r_in.get_direction()), rec.normal_surface);

        *attenuation = self.albedo;
        *scattered = Ray::new(rec.p, reflected + self.fuzz * vec3::random_in_unit_sphere());
        vec3::dot(scattered.get_direction(), rec.normal_surface) > 0.0
    }
}
