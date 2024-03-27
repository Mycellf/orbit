use crate::app::App;
use macroquad::prelude::*;
use nalgebra::{vector, Point2, UnitComplex, Vector2};

#[derive(Clone, Copy, Debug)]
pub struct Projectile {
    pub position: Point2<f32>,
    pub angle: UnitComplex<f32>,
    pub speed: f32,
    pub speed_exp_base: f32,
    pub lifetime: f32,
    pub age: f32,
    pub size: Vector2<f32>,
    pub color: Color,
}

impl Projectile {
    pub fn from_speed(
        speed: f32,
        speed_multiplier: f32,
        angle: UnitComplex<f32>,
        position: Point2<f32>,
        size: Vector2<f32>,
        lifetime: f32,
        color: Color,
    ) -> Self {
        let age = 0.0;
        Self {
            position,
            angle,
            speed,
            speed_exp_base: speed_multiplier,
            lifetime,
            age,
            size,
            color,
        }
    }

    pub fn update(&mut self, delta_seconds: f32, app: &mut App) -> Option<()> {
        self.age += delta_seconds;
        if self.age >= self.lifetime {
            return None;
        }

        if self.speed_exp_base == 1.0 {
            self.position += self.velocity() * delta_seconds;
        } else {
            self.position += self.distance_ahead(
                self.speed * (self.speed_exp_base.powf(delta_seconds) - 1.0)
                    / self.speed_exp_base.ln(),
            );
            self.speed *= self.speed_exp_base.powf(delta_seconds);
        }

        Some(())
    }

    pub fn draw(&self, frame_time: f32) {
        draw_rectangle_ex(
            self.position.x,
            self.position.y,
            self.size.y.max(self.speed * frame_time),
            self.size.x,
            DrawRectangleParams {
                offset: vec2(1.0, 0.5),
                rotation: self.angle.angle(),
                color: Color {
                    a: if self.age < 0.25 {
                        self.age / 0.25
                    } else {
                        1.0
                    },
                    ..self.color
                },
            },
        );
    }

    pub fn velocity(&self) -> Vector2<f32> {
        self.distance_ahead(self.speed)
    }

    pub fn distance_ahead(&self, distance: f32) -> Vector2<f32> {
        vector![self.angle.re, self.angle.im] * distance
    }
}
