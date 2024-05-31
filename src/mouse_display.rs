use macroquad::prelude::*;
use nalgebra::{point, Point2, UnitComplex};

pub struct MouseDisplay {
    pub radius: f32,
    pub center_angle: f32,
    pub center_speed: f32,
    pub ring_angle: f32,
    pub ring_speed: f32,
    pub position: Point2<f32>,
    pub color: Color,
    pub size_boost: u16,
    pub center_effect: u16,
    pub corner_effects: [u16; 4],
}

impl MouseDisplay {
    pub fn from_speed(center_speed: f32, ring_speed: f32) -> Self {
        let radius = 0.0;
        let center_angle = 0.0;
        let ring_angle = 0.0;
        let position = point![0.0, 0.0];
        let color = Default::default();
        let size_boost = 0;
        let center_effect = 0;
        let corner_effects = [0; 4];
        Self {
            radius,
            center_angle,
            center_speed,
            ring_angle,
            ring_speed,
            position,
            color,
            size_boost,
            center_effect,
            corner_effects,
        }
    }

    pub fn update_mouse_position(&mut self, camera: &Camera2D) {
        let position = mouse_position_local() / camera.zoom + camera.target;
        self.position = position.into();
    }

    pub fn set_effects_from_ring(&mut self, ring: &crate::components::ArmorRing) {
        for (armor, effect) in (&ring.armor)
            .into_iter()
            .rev()
            .zip(&mut self.corner_effects)
        {
            if let Some(armor) = armor {
                *effect = armor.hit_effect;
            } else {
                *effect = u16::MAX;
            }
        }
    }

    pub fn set_effects_from_empty_ring(&mut self) {
        for effect in &mut self.corner_effects {
            *effect = u16::MAX;
        }
    }

    pub fn draw(&self) {
        use std::f32::consts::{PI, SQRT_2};

        draw_rectangle_ex(
            self.position.x,
            self.position.y,
            1.0,
            1.0,
            DrawRectangleParams {
                offset: vec2(0.5, 0.5),
                rotation: self.center_angle,
                color: Color {
                    a: 1.0 - self.center_effect as f32 / u16::MAX as f32,
                    ..WHITE
                },
            },
        );

        let size_boost = self.size_boost as f32 / u16::MAX as f32 * 1.25;

        let angle = UnitComplex::new(self.ring_angle + PI / 4.0);
        let radius = self.radius + 0.5 + (size_boost * SQRT_2 / 2.0);
        for (i, (x, y)) in get_rotations_of(angle.re * radius, angle.im * radius)
            .into_iter()
            .enumerate()
        {
            let color = if self.corner_effects[i] < u16::MAX {
                let multiplier = 1.0 - self.corner_effects[i] as f32 / u16::MAX as f32;
                Color {
                    a: multiplier,
                    ..self.color
                }
            } else {
                Color {
                    a: self.radius.min(0.6),
                    ..self.color
                }
            };
            draw_rectangle_ex(
                self.position.x + x,
                self.position.y + y,
                1.0 + size_boost,
                1.0 + size_boost,
                DrawRectangleParams {
                    offset: vec2(0.5, 0.5),
                    rotation: self.ring_angle,
                    color,
                },
            );
        }
    }
}

/// Generates four clockwise rotations of x and y
fn get_rotations_of<T>(x: T, y: T) -> [(T, T); 4]
where
    T: std::ops::Neg<Output = T> + Copy,
{
    [(x, y), (y, -x), (-x, -y), (-y, x)]
}
