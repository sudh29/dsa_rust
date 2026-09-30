// src/math/geometry.rs - Geometry calculations module

use std::f64::consts::PI;

pub fn circle_area(radius: f64) -> f64 {
    PI * radius * radius
}

pub fn circle_circumference(radius: f64) -> f64 {
    2.0 * PI * radius
}

pub fn rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}

pub fn rectangle_perimeter(width: f64, height: f64) -> f64 {
    2.0 * (width + height)
}

pub fn triangle_area(base: f64, height: f64) -> f64 {
    (base * height) / 2.0
}

pub fn triangle_perimeter(a: f64, b: f64, c: f64) -> f64 {
    a + b + c
}

pub fn sphere_volume(radius: f64) -> f64 {
    (4.0 / 3.0) * PI * radius.powi(3)
}
