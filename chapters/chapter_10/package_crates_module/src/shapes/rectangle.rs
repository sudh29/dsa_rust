// src/shapes/rectangle.rs - Rectangle shape module

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    /// Creates a new Rectangle
    pub fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    /// Calculates the area of the rectangle
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    /// Calculates the perimeter of the rectangle
    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    /// Calculates the diagonal of the rectangle
    pub fn diagonal(&self) -> f64 {
        (self.width.powi(2) + self.height.powi(2)).sqrt()
    }

    /// Scales the rectangle by a factor
    pub fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }

    /// Checks if rectangle is a square
    pub fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Rectangle: {} x {} (Area: {:.2})",
            self.width,
            self.height,
            self.area()
        )
    }
}
