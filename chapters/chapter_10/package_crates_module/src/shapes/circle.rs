// src/shapes/circle.rs - Circle shape module

use std::f64::consts::PI;

pub struct Circle {
    pub radius: f64,
}

impl Circle {
    /// Creates a new Circle
    pub fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    /// Calculates the area of the circle
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    /// Calculates the circumference of the circle
    pub fn circumference(&self) -> f64 {
        2.0 * PI * self.radius
    }

    /// Calculates the diameter of the circle
    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }

    /// Scales the circle by a factor
    pub fn scale(&mut self, factor: f64) {
        self.radius *= factor;
    }

    /// Checks if a point is inside the circle
    pub fn contains_point(&self, x: f64, y: f64) -> bool {
        (x * x + y * y).sqrt() <= self.radius
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Circle: radius {} (Area: {:.2})",
            self.radius,
            self.area()
        )
    }
}
