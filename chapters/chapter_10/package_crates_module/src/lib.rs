// lib.rs - Library crate root
// This file demonstrates how to organize modules across multiple files

pub mod greetings;
pub mod math;
pub mod shapes;

// Re-export commonly used items for easier access
pub use math::arithmetic;
pub use math::geometry;
pub use shapes::{Circle, Rectangle};
