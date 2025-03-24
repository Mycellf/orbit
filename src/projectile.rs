use crate::{app::App, collision::Rectangle, components::Armor, controller::Team, entity::Entity};
use macroquad::prelude::*;
use nalgebra::{Point2, UnitComplex, Vector2, distance_squared, vector};
use thunderdome::Index;

#[derive(Clone, Copy, Debug)]
pub struct Projectile {
    pub position: Point2<f32>,
    pub angle: UnitComplex<f32>,
    pub initial_speed: f32,
    pub speed_exp_base: f32,
    pub lifetime: f32,
    pub age: f32,
    pub size: Vector2<f32>,
    pub color: Color,
    pub sender: Index,
    pub team: Option<Team>,
    pub previous_displacement: f32,
}

impl Projectile {
    pub fn from_speed(
        initial_speed: f32,
        speed_multiplier: f32,
        angle: UnitComplex<f32>,
        position: Point2<f32>,
        size: Vector2<f32>,
        lifetime: f32,
        color: Color,
        sender: Index,
        team: Option<Team>,
    ) -> Self {
        let age = 0.0;
        Self {
            position,
            angle,
            initial_speed,
            speed_exp_base: speed_multiplier,
            lifetime,
            age,
            size,
            color,
            sender,
            team,
            previous_displacement: 0.0,
        }
    }

    /// Returning `None` indicates a request for deletion.
    pub fn update(&mut self, delta_seconds: f32, app: &mut App) -> Option<()> {
        let previous_age = self.age;

        self.age += delta_seconds;
        if self.age >= self.lifetime {
            return None;
        }

        // Motion
        if self.speed_exp_base == 1.0 {
            self.position += self.velocity() * delta_seconds;

            self.previous_displacement = self.initial_speed * delta_seconds;
        } else {
            let displacement = self.initial_speed
                * (self.speed_exp_base.powf(self.age) - self.speed_exp_base.powf(previous_age))
                / self.speed_exp_base.ln();
            self.position += self.distance_ahead(displacement);

            self.previous_displacement = displacement;
        }

        // Collision
        let collider = self.get_collider();

        let mut hit = None;
        for (index, entity) in &mut app.entities {
            if Some(entity.team) != self.team
                && self
                    .check_collisions_with_entity(&collider, entity, self.angle)
                    .is_some()
            {
                hit = Some(index);
                break;
            }
        }
        if let Some(hit) = hit {
            if app.entities[hit].check_deletion().is_none() {
                app.entities.remove(hit);
            }
            None
        } else {
            Some(())
        }
    }

    pub fn draw(&self) {
        const FADE_IN_TIME: f32 = 0.25;
        const FADE_OUT_TIME: f32 = 0.1;

        draw_rectangle_ex(
            self.position.x,
            self.position.y,
            self.size.y.max(self.previous_displacement),
            self.size.x,
            DrawRectangleParams {
                offset: vec2(1.0, 0.5),
                rotation: self.angle.angle(),
                color: Color {
                    a: if self.age < FADE_IN_TIME {
                        self.age / FADE_IN_TIME
                    } else if self.age > self.lifetime - FADE_OUT_TIME {
                        (self.lifetime - self.age) / FADE_OUT_TIME
                    } else {
                        1.0
                    },
                    ..self.color
                },
            },
        );
    }

    pub fn velocity(&self) -> Vector2<f32> {
        self.distance_ahead(self.initial_speed)
    }

    pub fn distance_ahead(&self, distance: f32) -> Vector2<f32> {
        self.angle * vector![distance, 0.0]
    }

    pub fn check_collisions_with_entity(
        &self,
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
            if let Some(controller) = &mut entity.controller {
                controller.alert(self.sender);
            }
            return Some(());
        }

        None
    }

    /// Note that this factors in the previous displacement of the projectile
    pub fn get_collider(&self) -> Rectangle {
        Rectangle::from_dimensions(
            self.position,
            vector![self.size.y + self.previous_displacement, self.size.x],
            vector![1.0, 0.5],
            self.angle,
        )
    }
}
