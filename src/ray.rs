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
        let t: f64 = self.hits_sphere(Point3::new(0.0, 0.0, -1.0), 0.5);
        if t > 0.0 {
            let n = vec3::unit_vector(self.at(t) - Vec3::new(0.0, 0.0, -1.0));
            return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
        }
        let unit_direction = vec3::unit_vector(self.get_direction());
        let t: f64 = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    /*
       This function checks if this ray hits the radius of a given sphere based on the quadratic formula.
       If discriminant is a negative value, we immediately say that the ray does not hit the sphere.
       Else if zero or positive value, it means that the ray hits the sphere, we continue calculating the quadratic formula and return this result.
    */
    pub fn hits_sphere(&self, center: Point3, radius: f64) -> f64 {
        let oc = self.orig - center;
        let a = self.dir.length_squared();
        let half_b = vec3::dot(oc, self.dir);
        let c = oc.length_squared() - radius * radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            // discriminant is a negative value, no need to proceed with calculating since it does not interact with the object.
            -1.0
        } else {
            // continuing to calculate using the quadratic formula
            (-half_b - f64::sqrt(discriminant)) / a
        }
    }
}
