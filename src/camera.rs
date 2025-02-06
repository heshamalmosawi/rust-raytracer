use super::util::ASPECT_RATIO;
use crate::geometry::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    origin: Point3,            // the position of the camera in the 3D space
    lower_left_corner: Point3, // the point in 3D space where the camera visible area begins (lower left corner)
    width: Vec3,
    height: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let viewport_height = 2.0;
        let viewport_width = ASPECT_RATIO * viewport_height;
        let focal_length = 1.0; // distance from camera's origin

        let origin = Point3::new(0.0, 0.0, 0.0);

        // converting viewports to vectors
        let height = Vec3::new(0.0, viewport_height, 0.0);
        let width = Vec3::new(viewport_width, 0.0, 0.0);
        let lower_left_corner =
            origin - (width / 2.0) - (height / 2.0) - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            width,
            height,
        }
    }

    // this function generates a ray frmo the camera, given screen coordinates
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            // bottom left corner with horizontal and vertical offset, minus the origin point of the camera to get the direction vector of the ray
            self.lower_left_corner + u * self.width + v * self.height - self.origin,
        )
    }
}
