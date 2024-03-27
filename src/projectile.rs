use crate::app::App;
use macroquad::prelude::*;
use nalgebra::{vector, Point2, UnitComplex, Vector2};

#[derive(Clone, Copy, Debug)]
pub struct Projectile {
    pub position: Point2<f32>,
    pub angle: UnitComplex<f32>,
    pub speed: f32,
    pub lifetime: f32,
    pub age: f32,
    pub size: Vector2<f32>,
    pub color: Color,
}

impl Projectile {
    pub fn from_speed(
        speed: f32,
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

        self.position += self.velocity() * delta_seconds;

        Some(())
    }

    pub fn draw(&self) {
        draw_rectangle_ex(
            self.position.x,
            self.position.y,
            self.size.y,
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
        vector![self.angle.re, self.angle.im] * self.speed
    }
}
