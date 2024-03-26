use macroquad::prelude::*;
use nalgebra::{point, Point2};

pub struct MouseDisplay {
    radius: f32,
    center_angle: f32,
    center_speed: f32,
    ring_angle: f32,
    ring_speed: f32,
    position: Point2<f32>,
}

impl MouseDisplay {
    pub fn from_speed(center_speed: f32, ring_speed: f32) -> Self {
        let radius = 0.0;
        let center_angle = 0.0;
        let ring_angle = 0.0;
        let position = point![0.0, 0.0];
        Self {
            radius,
            center_angle,
            center_speed,
            ring_angle,
            ring_speed,
            position,
        }
    }

    pub fn update_mouse_position(&mut self, camera: &Camera2D) {
        let position = mouse_position_local() / camera.zoom + camera.target;
        self.position = position.into();
    }

    pub fn update(&mut self, delta_seconds: f32) {
        use std::f32::consts::PI;
        self.center_angle += self.center_speed * delta_seconds;
        self.center_angle %= PI / 2.0;
        self.ring_angle += self.ring_speed * delta_seconds;
        self.ring_angle %= PI / 2.0;

        if is_key_down(KeyCode::Space) {
            self.radius += delta_seconds * 2.0;
        } else {
            self.radius -= delta_seconds;
        }

        if self.radius < 0.0 {
            self.radius = 0.0;
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
        for (x, y) in get_rotations_of(cos * radius, sin * radius) {
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

/// Generates four clockwise rotations of x and y
fn get_rotations_of<T>(x: T, y: T) -> [(T, T); 4]
where
    T: std::ops::Neg<Output = T> + Copy,
{
    [(x, y), (y, -x), (-x, -y), (-y, x)]
}
