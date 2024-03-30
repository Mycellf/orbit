use crate::collision::Rectangle;
use macroquad::prelude::*;
use nalgebra::{vector, Point2, UnitComplex, Vector2};
use std::num::NonZeroU16;

#[derive(Clone, Debug)]
pub struct ArmorRing {
    pub armor: Vec<Option<Armor>>,
    pub radius: f32,
    pub angle: f32,
    pub speed: f32,
}

impl ArmorRing {
    pub fn from_size(
        size: Vector2<f32>,
        health: u16,
        count: usize,
        radius: f32,
        speed: f32,
    ) -> Self {
        let health = NonZeroU16::new(health).unwrap();
        let armor = (0..count).map(|_| Some(Armor { size, health })).collect();
        let angle = 0.0;
        Self {
            armor,
            radius,
            angle,
            speed,
        }
    }

    pub fn draw_around(&self, position: Point2<f32>, color: Color) {
        let mut angle = self.angle;
        let increment = self.get_increment();

        for armor in &self.armor {
            if let Some(Armor { size, .. }) = armor {
                let angle_complex = UnitComplex::new(angle);
                draw_rectangle_ex(
                    position.x + self.radius * angle_complex.re,
                    position.y + self.radius * angle_complex.im,
                    size.y,
                    size.x,
                    DrawRectangleParams {
                        offset: vec2(0.0, 0.5),
                        rotation: angle,
                        color,
                    },
                );
            }
            angle += increment;
        }
    }

    pub fn update_angle(&mut self, delta_seconds: f32) {
        use std::f32::consts::PI;
        self.angle += self.speed * delta_seconds;
        self.angle %= 2.0 * PI;
    }

    pub fn get_full_radius_squared(&self) -> f32 {
        (&self.armor)
            .into_iter()
            .filter_map(|&a| a)
            .map(|a| a.get_radius_squared(self.radius))
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap_or(0.0)
    }

    pub fn get_increment(&self) -> f32 {
        use std::f32::consts::PI;
        (2.0 * PI) / self.armor.len() as f32
    }

    pub fn get_colliders(&self, position: Point2<f32>) -> Vec<Option<Rectangle>> {
        let increment = self.get_increment();
        (&self.armor)
            .into_iter()
            .enumerate()
            .map(|(i, armor)| match armor {
                Some(armor) => {
                    let angle = UnitComplex::new(increment * i as f32 + self.angle);
                    Some(Rectangle::from_dimensions(
                        position + angle * vector![self.radius, 0.0],
                        vector![armor.size.y, armor.size.x],
                        vector![0.0, 0.5],
                        angle,
                    ))
                }
                None => None,
            })
            .collect()
    }

    /// Returns a vector of colliders associated with their armor piece.
    pub fn get_colliders_zip(
        &mut self,
        position: Point2<f32>,
    ) -> Vec<(Rectangle, &mut Option<Armor>)> {
        self.get_colliders(position)
            .into_iter()
            .zip(&mut *self.armor)
            .map(|(r, a)| Some((r?, a)))
            .filter_map(|a| a)
            .collect()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Armor {
    pub size: Vector2<f32>,
    pub health: NonZeroU16,
}

impl Armor {
    pub fn damage(reference: &mut Option<Armor>, damage: u16) {
        if let Some(armor) = reference {
            match NonZeroU16::new(armor.health.get() - damage) {
                Some(health) => {
                    armor.health = health;
                }
                None => {
                    *reference = None;
                    return;
                }
            }
        }
    }

    pub fn get_radius_squared(&self, radius: f32) -> f32 {
        let height = self.size.y + radius;
        height * height + self.size.x * self.size.x / 4.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Center {
    pub size: Vector2<f32>,
    pub health: NonZeroU16,
    pub angle: f32,
    pub speed: f32,
}

impl Center {
    pub fn from_size(size: Vector2<f32>, health: u16, speed: f32) -> Self {
        let health = NonZeroU16::new(health).unwrap();
        let angle = 0.0;
        Self {
            size,
            health,
            angle,
            speed,
        }
    }

    pub fn draw_around(&self, position: Point2<f32>, color: Color) {
        draw_rectangle_ex(
            position.x,
            position.y,
            self.size.x,
            self.size.y,
            DrawRectangleParams {
                offset: vec2(0.5, 0.5),
                rotation: self.angle,
                color,
            },
        );
    }

    pub fn update_angle(&mut self, delta_seconds: f32) {
        use std::f32::consts::PI;
        self.angle += self.speed * delta_seconds;
        self.angle %= 2.0 * PI;
    }

    pub fn get_radius_squared(&self) -> f32 {
        (self.size.x * self.size.x + self.size.y * self.size.y) / 4.0
    }

    pub fn get_collider(&self, position: Point2<f32>) -> Rectangle {
        Rectangle::from_dimensions(
            position,
            self.size,
            vector![0.5, 0.5],
            UnitComplex::new(self.angle),
        )
    }
}
