use rand::Rng;

// Constants
pub use std::f64::consts::PI;
pub use std::f64::INFINITY;

pub const ASPECT_RATIO: f64 = 3.0 / 2.0;
pub const IMAGE_WIDTH: i32 = 1200;
pub const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
pub const SAMPLES_PER_PIXEL: i32 = 100;

// Utility functions
// converts degrees to radian measurements
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

// generate random f64
pub fn random_double() -> f64 {
    rand::rng().random()
}

// generate random f64 in a given range from [min..max]
pub fn random_double_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}