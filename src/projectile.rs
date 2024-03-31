use crate::{app::App, collision::Rectangle, components::Armor, entity::Entity};
use macroquad::prelude::*;
use nalgebra::{distance_squared, vector, Point2, UnitComplex, Vector2};

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

    /// Returning `None` indicates a request for deletion.
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

        let collider = self.get_collider(delta_seconds);
        for i in (0..app.entities.len()).rev() {
            let entity = &mut app.entities[i];
            if entity.color != self.color
                && Self::check_collisions_with_entity(&collider, entity, self.angle).is_some()
            {
                if entity.check_deletion().is_none() {
                    app.entities.swap_remove(i);
                }
                return None;
            }
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
        self.angle * vector![distance, 0.0]
    }

    pub fn check_collisions_with_entity(
        collider: &Rectangle,
        entity: &mut Entity,
        direction: UnitComplex<f32>,
    ) -> Option<()> {
        let center = collider.center();
        if distance_squared(&center, &entity.position)
            > (collider.radius_squared().sqrt() + entity.radius).powi(2)
        {
            return None;
        }

        let direction = vector![direction.re, direction.im];
        if let Some((_, armor)) = (&mut entity.rings)
            .into_iter()
            .flat_map(|ring| ring.get_colliders_zip(entity.position))
            .chain(vec![(
                entity.center.get_collider(entity.position),
                &mut entity.center.armor,
            )])
            .filter(|(rect, _)| collider.is_colliding(rect))
            .map(|(rect, armor)| {
                // sort collisions by closest point
                (
                    rect.corners
                        .into_iter()
                        .map(|c| (c - center).dot(&direction))
                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap(),
                    armor,
                )
            })
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        {
            Armor::damage(armor, 1);
            return Some(());
        }

        None
    }

    /// Note that this factors in the speed of the projectile to make its hitbox longer. To
    /// disable this, pass 0.0 to `delta_seconds`.
    pub fn get_collider(&self, delta_seconds: f32) -> Rectangle {
        Rectangle::from_dimensions(
            self.position,
            vector![self.size.y + self.speed * delta_seconds, self.size.x],
            vector![1.0, 0.5],
            self.angle,
        )
    }
}
