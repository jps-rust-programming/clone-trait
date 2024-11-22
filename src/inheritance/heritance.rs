use super::traits::{colorful::Colorful, shape::Shape};
pub struct Circle {
    pub radius: f64,
    pub color: String,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius
    }
}

impl Colorful for Circle {
    fn color(&self) -> &str {
        &self.color // Return the color of the circle
    }
}
