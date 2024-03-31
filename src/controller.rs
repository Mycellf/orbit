use crate::{
    app::App,
    entity::Entity,
    input::{InputAxis, InputButton},
    projectile::Projectile,
};
use macroquad::prelude::*;
use nalgebra::{vector, Complex, UnitComplex, Vector2};

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
        drag: f32,
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
                    aim.angle() + rand::gen_range(-0.1, 0.1) * (*shooting_speed - 1.0),
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
                if let Some(ring) = entity.rings.get(0) {
                    app.mouse.ring_angle = ring.angle - PI * 3.0 / 4.0;
                    app.mouse.set_active_from_ring(ring);
                } else {
                    app.mouse.ring_angle = entity.center.angle * -0.5 - (PI * 3.0 / 4.0);
                    app.mouse.active_corners = 0;
                }
                app.mouse.radius = (*shooting_speed - 1.0)
                    * (length(entity.position - app.mouse.position))
                    * 0.125;
                app.mouse.radius = app.mouse.radius.max(0.0);
                app.mouse.color = entity.color;
                app.mouse.size_boost = (*cooldown * *shooting_speed * u16::MAX as f32) as u16;
            }
            Self::Enemy { speed, drag } => {
                let direction = UnitComplex::new(0.0);
                // let impulse =
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

    pub fn enemy(speed: f32, drag: f32) -> Self {
        Self::Enemy { speed, drag }
    }
}

fn displacement_from_angle(angle: UnitComplex<f32>, distance: f32) -> Vector2<f32> {
    vector![angle.re, angle.im] * distance
}

fn length(vector: Vector2<f32>) -> f32 {
    (vector.x * vector.x + vector.y * vector.y).sqrt()
}
