use crate::{
    color::Color,
    vec3::{self, Point3, Vec3},
};

#[derive(Default)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub fn get_origin(&self) -> Point3 {
        self.orig
    }

    pub fn get_direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }

    /*
        This function returns a linearly blend of white and blue depending on the height of the 'y' coordinate, to be used for the sky.
        It does this by getting unit vector and normalizing it from range (-1, 1) to (0, 1), and applying it to the linear blend of white/blue.
        `blendedValue` = (1 - t) * startValue + t * endValue
    */
    pub fn get_ray_collor(&self) -> Color {
        let unit_direction = vec3::unit_vector(self.get_direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
