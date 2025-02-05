
// Constants
pub use std::f64::consts::PI;
pub use std::f64::INFINITY;
pub const IMAGE_WIDTH: i32 = 400;
pub const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
 
 
// Utility functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}