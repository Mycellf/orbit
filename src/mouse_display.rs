use macroquad::prelude::*;
use nalgebra::{point, Point2};

pub struct MouseDisplay {
    pub radius: f32,
    pub center_angle: f32,
    pub center_speed: f32,
    pub ring_angle: f32,
    pub ring_speed: f32,
    pub position: Point2<f32>,
    pub blink_time: f32,
    pub color: Color,
    pub active_corners: u8,
}

impl MouseDisplay {
    pub fn from_speed(center_speed: f32, ring_speed: f32) -> Self {
        let radius = 0.0;
        let center_angle = 0.0;
        let ring_angle = 0.0;
        let position = point![0.0, 0.0];
        let blink_time = 0.0;
        let color = Default::default();
        let active_corners = 0xf;
        Self {
            radius,
            center_angle,
            center_speed,
            ring_angle,
            ring_speed,
            position,
            blink_time,
            color,
            active_corners,
        }
    }

    pub fn update_mouse_position(&mut self, camera: &Camera2D) {
        let position = mouse_position_local() / camera.zoom + camera.target;
        self.position = position.into();
    }

    pub fn update(&mut self, delta_seconds: f32) {
        self.blink_time += delta_seconds;
        if self.blink_time > 1.0 {
            self.blink_time = -1.0;
        }
    }

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
        use std::f32::consts::{PI, SQRT_2};

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

        let force = self.blink_time >= 0.0;

        let cos = (self.ring_angle + PI / 4.0).cos();
        let sin = (self.ring_angle + PI / 4.0).sin();
        let radius = self.radius + 0.5;
        for (i, (x, y)) in get_rotations_of(cos * radius, sin * radius)
            .into_iter()
            .enumerate()
        {
            let color = if force || (self.active_corners >> i) & 1 > 0 {
                self.color
            } else {
                Color {
                    a: self.radius.min(0.6),
                    ..self.color
                }
            };
            draw_rectangle_ex(
                self.position.x + x,
                self.position.y + y,
                1.0,
                1.0,
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
