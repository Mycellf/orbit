use macroquad::prelude::*;
use nalgebra::{vector, Point2, Vector2};
use std::num::NonZeroU8;

use crate::input::InputAxis;

pub struct Entity {
    pub rings: Vec<ArmorRing>,
    pub center: Center,
    pub position: Point2<f32>,
    pub aim: Option<f32>,
    pub radius: f32,
    pub color: Color,
    pub controller: Option<Controller>,
}

impl Entity {
    pub fn from_rings(
        position: Point2<f32>,
        color: Color,
        center: Center,
        rings: Vec<ArmorRing>,
        controller: Option<Controller>,
    ) -> Self {
        let aim = None;
        let radius = Self::get_radius_of(&rings, &center);
        Self {
            rings,
            center,
            position,
            aim,
            radius,
            color,
            controller,
        }
    }

    pub fn draw(&self) {
        self.center.draw_around(self.position, WHITE);
        for ring in &*self.rings {
            ring.draw_around(self.position, self.color);
        }
    }

    pub fn update(&mut self, delta_seconds: f32) {
        self.center.update_angle(delta_seconds);
        for ring in &mut *self.rings {
            ring.update_angle(delta_seconds);
        }

        Controller::update(self, delta_seconds);
    }

    pub fn get_full_radius(&self) -> f32 {
        Self::get_radius_of(&self.rings, &self.center)
    }

    fn get_radius_of(rings: &Vec<ArmorRing>, center: &Center) -> f32 {
        rings
            .into_iter()
            .map(|r| r.get_full_radius())
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap_or_else(|| center.size.x.max(center.size.y))
    }
}

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
        use std::f32::consts::PI;
        let mut angle = self.angle;
        let increment = (2.0 * PI) / self.armor.len() as f32;

        for armor in &self.armor {
            if let Some(Armor { size, .. }) = armor {
                draw_rectangle_ex(
                    position.x + self.radius * angle.cos(),
                    position.y + self.radius * angle.sin(),
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

    pub fn get_full_radius(&self) -> f32 {
        let max_height = (&self.armor)
            .into_iter()
            .filter_map(|&a| a)
            .map(|a| a.size.y)
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap_or(0.0);
        max_height + self.radius
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Armor {
    pub size: Vector2<f32>,
    pub health: NonZeroU8,
}

pub struct Center {
    pub size: Vector2<f32>,
    pub health: NonZeroU8,
    pub angle: f32,
    pub speed: f32,
}

impl Center {
    pub fn from_size(size: Vector2<f32>, health: u8, speed: f32) -> Self {
        let health = NonZeroU8::new(health).unwrap();
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
}

#[derive(Clone, Debug)]
pub enum Controller {
    Player {
        speed: f32,
        x_control: InputAxis,
        y_control: InputAxis,
    },
}

impl Controller {
    pub fn update(entity: &mut Entity, delta_seconds: f32) -> Option<()> {
        let controller = entity.controller.as_mut()?;
        match controller {
            Self::Player {
                speed,
                x_control,
                y_control,
            } => {
                x_control.update_state();
                y_control.update_state();

                entity.position +=
                    vector![x_control.as_f32(), y_control.as_f32()] * (*speed * delta_seconds);
            }
        }
        Some(())
    }
}
