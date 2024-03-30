use macroquad::prelude::*;
use nalgebra::{point, Point2};

pub struct MouseDisplay {
    pub radius: f32,
    pub center_angle: f32,
    pub center_speed: f32,
    pub ring_angle: f32,
    pub ring_speed: f32,
    pub position: Point2<f32>,
    pub active_corners: u8,
}

impl MouseDisplay {
    pub fn from_speed(center_speed: f32, ring_speed: f32) -> Self {
        let radius = 0.0;
        let center_angle = 0.0;
        let ring_angle = 0.0;
        let position = point![0.0, 0.0];
        let active_corners = 0xf;
        Self {
            radius,
            center_angle,
            center_speed,
            ring_angle,
            ring_speed,
            position,
            active_corners,
        }
    }

    pub fn update_mouse_position(&mut self, camera: &Camera2D) {
        let position = mouse_position_local() / camera.zoom + camera.target;
        self.position = position.into();
    }

    pub fn update(&mut self, delta_seconds: f32) {}

    pub fn set_active_from_ring(&mut self, ring: &crate::components::ArmorRing) {
        self.active_corners = 0;
        for armor in &ring.armor {
            self.active_corners <<= 1;
            if armor.is_some() {
                self.active_corners |= 1;
            }
        }
    }

    pub fn draw(&self) {
        use std::f32::consts::PI;

        draw_rectangle_ex(
            self.position.x,
            self.position.y,
            1.0,
            1.0,
            DrawRectangleParams {
                offset: vec2(0.5, 0.5),
                rotation: self.center_angle,
                color: WHITE,
            },
        );

        let cos = (self.ring_angle + PI / 4.0).cos();
        let sin = (self.ring_angle + PI / 4.0).sin();
        let radius = self.radius + 0.5;
        for (i, (x, y)) in get_rotations_of(cos * radius, sin * radius)
            .into_iter()
            .enumerate()
        {
            if (self.active_corners >> i) & 1 > 0 {
                draw_rectangle_ex(
                    self.position.x + x,
                    self.position.y + y,
                    1.0,
                    1.0,
                    DrawRectangleParams {
                        offset: vec2(0.5, 0.5),
                        rotation: self.ring_angle,
                        color: Color::from_hex(0x0000ff),
                    },
                );
            }
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
