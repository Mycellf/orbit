use std::ops::Range;

use nalgebra::{vector, UnitComplex, Vector2};

pub fn length(vector: Vector2<f32>) -> f32 {
    length_squared(vector).sqrt()
}

pub fn length_squared(vector: Vector2<f32>) -> f32 {
    vector.x.powi(2) + vector.y.powi(2)
}

pub fn lerp(range: &Range<f32>, interpolation: f32) -> f32 {
    range.start + (range.end - range.start) * interpolation
}

pub fn displacement_from_angle(angle: UnitComplex<f32>, distance: f32) -> Vector2<f32> {
    vector![angle.re, angle.im] * distance
}
