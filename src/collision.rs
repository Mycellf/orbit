use macroquad::prelude::*;
use nalgebra::{center, distance_squared, vector, Point2, UnitComplex, Vector2};

/// Corners are assumed to be in either clockwise or counter-clockwise order.
#[derive(Clone, Debug)]
pub struct Rectangle {
    pub corners: [Point2<f32>; 4],
    pub edges: [Vector2<f32>; 4],
}

impl Rectangle {
    pub fn from_dimensions(
        center: Point2<f32>,
        size: Vector2<f32>,
        offset: Vector2<f32>,
        angle: UnitComplex<f32>,
    ) -> Self {
        let rot_size_x = angle * vector![size.x, 0.0];
        let rot_size_y = angle * vector![0.0, size.y];
        let corner = center - rot_size_x * offset.x - rot_size_y * offset.y;
        Self::from_corners([
            corner,
            corner + rot_size_x,
            corner + rot_size_x + rot_size_y,
            corner + rot_size_y,
        ])
    }

    /// Corners are assumed to be in either clockwise or counter-clockwise order.
    pub fn from_corners(corners: [Point2<f32>; 4]) -> Self {
        let edges = loop_indices().map(|(a, b)| corners[a] - corners[b]);
        Self { corners, edges }
    }

    pub fn is_colliding(&self, other: &Self) -> bool {
        self.check_collision_one_sided(other) && other.check_collision_one_sided(self)
    }

    fn check_collision_one_sided(&self, other: &Self) -> bool {
        // I hope I don't regret doing this with iterator syntax. (as far a code maintenance goes)
        !(self.corners.into_iter())
            .zip(self.edges)
            .any(|(offset, axis)| {
                (other.corners)
                    .into_iter()
                    .all(|corner| (corner - offset).dot(&axis) >= 0.0)
            })
    }

    pub fn draw_debug(&self) {
        let points: [_; 4] = loop_indices().map(|(a, b)| (&self.corners[a], &self.corners[b]));
        for (a, b) in points {
            draw_line(a.x, a.y, b.x, b.y, 0.1, MAGENTA);
        }
    }

    pub fn radius_squared(&self) -> f32 {
        distance_squared(&self.corners[0], &self.corners[2]) / 4.0
    }

    pub fn center(&self) -> Point2<f32> {
        center(&self.corners[0], &self.corners[2])
    }
}

/// For use with array::map.
/// It is rarely nececcary to specify the length of the output.
/// ```
/// assert_eq!(loop_indices(), [(0, 1), (1, 2), (2, 0)]);
/// ```
fn loop_indices<const N: usize>() -> [(usize, usize); N] {
    let mut array = [Default::default(); N];
    for i in 0..N {
        array[i] = (i, i + 1);
    }
    array[N - 1].1 = 0;
    array
}
