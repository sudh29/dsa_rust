// src/shapes/mod.rs - Shapes module root
// This file declares what's in the shapes module

pub mod circle;
pub mod rectangle;

// Re-export for easier access
pub use circle::Circle;
pub use rectangle::Rectangle;
