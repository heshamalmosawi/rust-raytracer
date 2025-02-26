use super::util::ASPECT_RATIO;
use crate::{
    geometry::{
        ray::Ray,
        vec3::{self, Point3, Vec3},
    },
    util,
};

pub struct Camera {
    origin: Point3,            // the position of the camera in the 3D space
    lower_left_corner: Point3, // the point in 3D space where the camera visible area begins (lower left corner)
    width: Vec3,
    height: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, field_of_view: f64, aperature: f64, focus_dist: f64) -> Self {
        let theta = util::degrees_to_radians(field_of_view);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;

        let viewport_width = ASPECT_RATIO * viewport_height;

        // three unit vectors:
        // u: the unit vector pointing to camera right
        // v: the unit vector pointing to upside direction
        // w: the unit vector pointing opposite the view direction
        

        let w = vec3::unit_vector(lookfrom - lookat);
        let u = vec3::unit_vector(vec3::cross(vup, w));
        let v = vec3::cross(w, u);

        let origin = lookfrom;

        // converting viewports to vectors
        let height = focus_dist * viewport_height * v;
        let width = focus_dist * viewport_width * u;        
        let lower_left_corner = origin - width / 2.0 - height / 2.0 - focus_dist * w;

        let lens_radius = aperature / 2.0;
        Self {
            origin,
            lower_left_corner,
            width,
            height,
            u,
            v,
            lens_radius
        }
    }

    // this function generates a ray from the camera, given screen coordinates
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            // bottom left corner with horizontal and vertical offset, minus the origin point of the camera to get the direction vector of the ray
            self.lower_left_corner + s * self.width + t * self.height - self.origin - offset,

        )
    }
}
