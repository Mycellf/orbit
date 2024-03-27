use nalgebra::{Point2, UnitComplex};

pub struct Projectile {
    position: Point2<f32>,
    angle: UnitComplex<f32>,
}
