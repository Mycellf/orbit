use crate::{
    app::App,
    entity::Entity,
    input::{InputAxis, InputButton},
    projectile::Projectile,
};
use macroquad::prelude::*;
use nalgebra::{vector, Complex, UnitComplex, Vector2};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum Controller {
    Player {
        speed: f32,
        x_control: InputAxis,
        y_control: InputAxis,
        shoot_control: Vec<InputButton>,
        cooldown: f32,
        shooting_speed: f32,
    },
    Enemy {
        speed: f32,
        min_range: f32,
        max_range: f32,
        target: Target,
        max_cooldown: f32,
        cooldown: f32,
    },
}

impl Controller {
    pub fn update(entity: &mut Entity, delta_seconds: f32, app: &mut App) -> Option<()> {
        let controller = entity.controller.as_mut()?;
        match controller {
            Self::Player {
                speed,
                x_control,
                y_control,
                shoot_control,
                cooldown,
                shooting_speed,
            } => {
                // Mouse aim
                let aim = app.mouse.position - entity.position;
                let aim = UnitComplex::from_complex(Complex::new(aim.x, aim.y));
                entity.aim = Some(aim);

                // Motion
                x_control.update_state();
                y_control.update_state();
                let input = vector![x_control.as_f32(), y_control.as_f32()];
                let input = if input.x == 0.0 {
                    input
                } else {
                    input.normalize()
                };
                entity.velocity = input * *speed;

                // Shooting
                let shoot_pressed = shoot_control.into_iter().any(|b| b.is_down());
                let nudged_aim = UnitComplex::new(
                    aim.angle() + rand::gen_range(-0.15, 0.15) * (*shooting_speed - 1.0),
                );

                if *cooldown > 0.0 {
                    *cooldown -= delta_seconds;
                }
                if *cooldown <= 0.0 && shoot_pressed {
                    *cooldown = 0.5 / *shooting_speed;
                    app.projectiles.push(Projectile::from_speed(
                        48.0,
                        50.0,
                        nudged_aim,
                        entity.position + displacement_from_angle(nudged_aim, entity.radius + 4.0),
                        vector![1.0, 4.0],
                        2.0,
                        entity.color,
                    ));
                }

                if shoot_pressed {
                    if *shooting_speed <= 2.0 {
                        *shooting_speed += delta_seconds * 0.5;
                    } else {
                        *shooting_speed = 2.0;
                    }
                } else if *shooting_speed > 1.0 {
                    *shooting_speed -= delta_seconds;
                } else {
                    *shooting_speed = 1.0;
                }

                // Sync Mouse
                use std::f32::consts::PI;
                app.mouse.center_angle = entity.center.angle;
                app.mouse.center_effect = entity.center.hit_effect;
                if let Some(ring) = entity.rings.get(0) {
                    app.mouse.ring_angle = ring.angle - PI * 3.0 / 4.0;
                    app.mouse.set_effects_from_ring(ring);
                } else {
                    app.mouse.ring_angle = entity.center.angle * -0.5 - (PI * 3.0 / 4.0);
                }
                app.mouse.radius = (*shooting_speed - 1.0)
                    * (length(entity.position - app.mouse.position))
                    * 0.125;
                app.mouse.radius = app.mouse.radius.max(0.0);
                app.mouse.color = entity.color;
                app.mouse.size_boost = (*cooldown * *shooting_speed * u16::MAX as f32) as u16;
            }
            Self::Enemy {
                speed,
                min_range,
                max_range,
                target,
                max_cooldown,
                cooldown,
            } => {
                entity.aim = None;
                let target_entity = match target.get(app) {
                    Some(target_entity) => target_entity,
                    None => {
                        let target_entity = (&app.entities)
                            .into_iter()
                            .find(|e| e.color != entity.color)?;
                        target.set(target_entity);

                        target_entity
                    }
                };

                let displacement = target_entity.position - entity.position;
                let multiplier = if length_squared(displacement) < *min_range * *min_range {
                    -1.0
                } else if length_squared(displacement) > *max_range * *max_range {
                    1.0
                } else {
                    0.0
                };
                let direction =
                    UnitComplex::from_complex(Complex::new(displacement.x, displacement.y));
                entity.aim = Some(direction);
                let impulse = direction * vector![*speed * multiplier, 0.0];
                entity.velocity = impulse;

                if *cooldown > 0.0 {
                    *cooldown -= delta_seconds;
                } else {
                    *cooldown = *max_cooldown;
                    app.projectiles.push(Projectile::from_speed(
                        48.0,
                        50.0,
                        direction,
                        entity.position + displacement_from_angle(direction, entity.radius + 4.0),
                        vector![1.0, 4.0],
                        2.0,
                        entity.color,
                    ));
                }
            }
        }
        Some(())
    }

    pub fn player(
        speed: f32,
        x_control: InputAxis,
        y_control: InputAxis,
        shoot_control: Vec<InputButton>,
    ) -> Self {
        let cooldown = 0.0;
        let shooting_speed = 1.0;
        Self::Player {
            speed,
            x_control,
            y_control,
            shoot_control,
            cooldown,
            shooting_speed,
        }
    }

    pub fn enemy(speed: f32, min_range: f32, max_range: f32, max_cooldown: f32) -> Self {
        let target = Target::default();
        let cooldown = max_cooldown;
        Self::Enemy {
            speed,
            min_range,
            max_range,
            target,
            max_cooldown,
            cooldown,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Target {
    pub uuid: Uuid,
    pub index: usize,
}

impl Target {
    pub fn from_uuid(uuid: Uuid) -> Self {
        let index = 0;
        Self { uuid, index }
    }

    pub fn get<'a>(&mut self, app: &'a App) -> Option<&'a Entity> {
        if let Some(entity) = app.entities.get(self.index) {
            if entity.uuid == self.uuid {
                return Some(entity);
            }
        }

        let (index, entity) = (&app.entities)
            .into_iter()
            .enumerate()
            .find(|e| e.1.uuid == self.uuid)?;
        self.index = index;
        Some(entity)
    }

    pub fn set(&mut self, entity: &Entity) {
        self.uuid = entity.uuid;
    }
}

impl Default for Target {
    /// Will not have any target
    fn default() -> Self {
        Self::from_uuid(uuid::Builder::from_random_bytes([0; 16]).into_uuid())
    }
}

fn displacement_from_angle(angle: UnitComplex<f32>, distance: f32) -> Vector2<f32> {
    vector![angle.re, angle.im] * distance
}

fn length(vector: Vector2<f32>) -> f32 {
    length_squared(vector).sqrt()
}

fn length_squared(vector: Vector2<f32>) -> f32 {
    vector.x * vector.x + vector.y * vector.y
}
