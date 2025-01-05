use crate::collision::Rectangle;
use macroquad::prelude::*;
use nalgebra::{vector, Point2, UnitComplex, Vector2};
use std::num::NonZeroU8;

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
        health: u8,
        count: usize,
        radius: f32,
        speed: f32,
    ) -> Self {
        let health = NonZeroU8::new(health).unwrap();
        let armor = (0..count)
            .map(|_| Some(Armor::from_size(size, health)))
            .collect();
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
            if let Some(armor) = armor {
                let angle_complex = UnitComplex::new(angle);
                draw_rectangle_ex(
                    position.x + self.radius * angle_complex.re,
                    position.y + self.radius * angle_complex.im,
                    (armor.get_health_ratio() * armor.size.y).max(0.125),
                    armor.size.x,
                    DrawRectangleParams {
                        offset: vec2(0.0, 0.5),
                        rotation: angle,
                        color: armor.modify_color(color),
                    },
                );
            }
            angle += increment;
        }
    }

    pub fn update(&mut self, delta_seconds: f32) {
        use std::f32::consts::TAU;
        self.angle += self.speed * delta_seconds;
        self.angle %= TAU;

        for armor in &mut self.armor {
            if let Some(armor) = armor {
                armor.update_hit_effect(delta_seconds);
            }
        }
    }

    pub fn get_full_radius_squared(&self) -> Option<f32> {
        (&self.armor)
            .into_iter()
            .filter_map(|&a| a)
            .map(|a| a.get_radius_squared(self.radius))
            .max_by(|x, y| x.partial_cmp(y).unwrap())
    }

    pub fn get_increment(&self) -> f32 {
        use std::f32::consts::TAU;
        TAU / self.armor.len() as f32
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
                        vector![armor.get_health_ratio() * armor.size.y, armor.size.x],
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

/// Note that accessing `size` or `health` will panic if `armor` is `None`. This should never be
/// the case unless the entity center is associated with is about to be deleted.
#[derive(Clone, Copy, Debug)]
pub struct Center {
    pub armor: Option<Armor>,
    pub angle: f32,
    pub speed: f32,
}

impl Center {
    pub fn from_size(size: Vector2<f32>, health: u8, speed: f32) -> Self {
        let health = NonZeroU8::new(health).unwrap();
        let armor = Some(Armor::from_size(size, health));
        let angle = 0.0;
        Self {
            armor,
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
                color: self.modify_color(color),
            },
        );

        if let Some(armor) = self.armor {
            if armor.health.get() < armor.max_health.get() {
                let health_proportion = armor.health.get() as f32 / armor.max_health.get() as f32;

                let hole_size = self.size * (health_proportion - 1.0) * 0.8;

                draw_rectangle_ex(
                    position.x,
                    position.y,
                    hole_size.x,
                    hole_size.y,
                    DrawRectangleParams {
                        offset: vec2(0.5, 0.5),
                        rotation: self.angle,
                        color: Color::from_hex(0x000000),
                    },
                );
            }
        }
    }

    pub fn update(&mut self, delta_seconds: f32) {
        use std::f32::consts::TAU;
        self.angle += self.speed * delta_seconds;
        self.angle %= TAU;
        self.update_hit_effect(delta_seconds);
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

impl std::ops::Deref for Center {
    type Target = Armor;

    fn deref(&self) -> &Self::Target {
        self.armor.as_ref().unwrap()
    }
}

impl std::ops::DerefMut for Center {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.armor.as_mut().unwrap()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Armor {
    pub size: Vector2<f32>,
    pub health: NonZeroU8,
    pub max_health: NonZeroU8,
    pub hit_effect: u16,
}

impl Armor {
    pub fn from_size(size: Vector2<f32>, health: NonZeroU8) -> Self {
        let hit_effect = 0;
        let max_health = health;
        Self {
            size,
            health,
            max_health,
            hit_effect,
        }
    }

    pub fn damage(reference: &mut Option<Armor>, damage: u8) {
        if let Some(armor) = reference {
            match NonZeroU8::new(armor.health.get().saturating_sub(damage)) {
                Some(health) => {
                    armor.health = health;
                    armor.hit_effect = u16::MAX / 4 * 3;
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

    pub fn modify_color(&self, color: Color) -> Color {
        let multiplier = 1.0 - self.hit_effect as f32 / u16::MAX as f32;
        Color {
            r: color.r * multiplier,
            g: color.g * multiplier,
            b: color.b * multiplier,
            a: color.a,
        }
    }

    pub fn update_hit_effect(&mut self, delta_seconds: f32) {
        self.hit_effect =
            (self.hit_effect).saturating_sub((2.0 * delta_seconds * u16::MAX as f32) as u16)
    }

    pub fn get_health_ratio(&self) -> f32 {
        self.health.get() as f32 / self.max_health.get() as f32
    }
}
