use std::rc::Rc;

use crate::geometry::ray::Ray;
use crate::geometry::vec3::{self, Point3, Vec3};
use crate::material::Material;

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3, // point of intersection
    pub normal_surface: Vec3, // normal to the surface at the point of intersection
    pub material: Option<Rc<dyn Material>>, // Material of object
    pub t: f64, // time of intersection
    pub hits_front: bool, // whether the ray is hitting the front or back face of the surface
}

impl HitRecord {
    pub fn new() -> Self {
        Default::default()
    }

    // this function sets the normal surface & front face to the object, it is used to determine whether the front or back face of a surface. 
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // if the dot product is negative, then the angle between them is greater than 90 degress (meaning the ray is coming from outside the surface)
        // else if negative then the ray is coming from inside the object (back face).
        self.hits_front = vec3::dot(r.get_direction(), outward_normal) < 0.0;
        self.normal_surface = if self.hits_front {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
