use crate::{
    app::App,
    entity::Entity,
    input::{InputAxis, InputButton},
    projectile::Projectile,
};
use macroquad::{color::Color, prelude::rand};
use nalgebra::{vector, Complex, Point2, UnitComplex, Vector2};
use std::ops::Range;
use thunderdome::Index;

#[derive(Clone, Debug)]
pub struct EntityController {
    pub motion: Option<MotionController>,
    pub shooting: Option<ShootingController>,
}

impl EntityController {
    pub fn update(entity: &mut Entity, index: Index, delta_seconds: f32, app: &mut App) {
        let controller = if let Some(controller) = entity.controller.as_mut() {
            controller
        } else {
            return;
        };

        if let Some(motion) = controller.motion.as_mut() {
            match motion {
                MotionController::Player {
                    x_control,
                    y_control,
                    speed,
                } => {
                    x_control.update_state();
                    y_control.update_state();
                    let input = vector![x_control.as_f32(), y_control.as_f32()];
                    let input = if input.x == 0.0 {
                        input
                    } else {
                        input.normalize()
                    };
                    entity.velocity = input * *speed;
                }
            }
        }

        if let Some(shooting) = controller.shooting.as_mut() {
            match shooting {
                ShootingController::Player {
                    shoot_control,
                    precise_shoot_control,
                    cooldown,
                    state,
                    speed,
                    precision,
                    delay,
                } => {
                    let shoot_input = shoot_control.into_iter().any(|b| b.is_down());
                    let precise_shoot_input =
                        precise_shoot_control.into_iter().any(|b| b.is_down());

                    let input = shoot_input || precise_shoot_input;
                    let accelerate = shoot_input;

                    let aim = app.mouse.position - entity.position;
                    let aim = UnitComplex::from_complex(Complex::new(aim.x, aim.y));
                    entity.aim = Some(aim);

                    *cooldown = (*cooldown - delta_seconds).max(0.0);
                    if input && *cooldown <= 0.0 {
                        *cooldown = lerp(speed, *state);
                        let nudged_aim = UnitComplex::new(
                            aim.angle() + rand::gen_range(-1.0, 1.0) * lerp(precision, *state),
                        );
                        insert_projectile(
                            app,
                            nudged_aim,
                            entity.position,
                            entity.radius + 4.0,
                            entity.color,
                            index,
                        );
                    }

                    *state += delta_seconds / if accelerate { delay.start } else { -delay.end };
                    *state = state.clamp(0.0, 1.0);

                    // Sync mouse
                    use std::f32::consts::PI;
                    app.mouse.center_angle = entity.center.angle;
                    app.mouse.center_effect = entity.center.hit_effect;
                    if let Some(ring) = entity.rings.get(0) {
                        app.mouse.ring_angle = ring.angle - PI * 3.0 / 4.0;
                        app.mouse.set_effects_from_ring(ring);
                    } else {
                        app.mouse.ring_angle = entity.center.angle * -0.5 - (PI * 3.0 / 4.0);
                        app.mouse.set_effects_from_empty_ring();
                    }
                    app.mouse.radius =
                        *state * (length(entity.position - app.mouse.position)) * 0.125;
                    app.mouse.radius = app.mouse.radius.max(0.0);
                    app.mouse.color = entity.color;
                    app.mouse.size_boost = (*cooldown * 1.0 * u16::MAX as f32) as u16;
                }
            }
        }
    }

    pub fn alert(&mut self, sender: Index) {}
}

#[derive(Clone, Debug)]
pub enum MotionController {
    Player {
        x_control: InputAxis,
        y_control: InputAxis,
        speed: f32,
    },
}

#[derive(Clone, Debug)]
pub enum ShootingController {
    Player {
        shoot_control: Vec<InputButton>,
        precise_shoot_control: Vec<InputButton>,
        cooldown: f32,
        state: f32,
        speed: Range<f32>,
        precision: Range<f32>,
        delay: Range<f32>,
    },
}

fn insert_projectile(
    app: &mut App,
    aim: UnitComplex<f32>,
    position: Point2<f32>,
    offset_radius: f32,
    color: Color,
    sender: Index,
) {
    app.projectiles.insert(Projectile::from_speed(
        48.0,
        50.0,
        aim,
        position + displacement_from_angle(aim, offset_radius),
        vector![1.0, 4.0],
        1.0,
        color,
        sender,
    ));
}

fn length(vector: Vector2<f32>) -> f32 {
    length_squared(vector).sqrt()
}

fn length_squared(vector: Vector2<f32>) -> f32 {
    vector.x.powi(2) + vector.y.powi(2)
}

fn lerp(range: &mut Range<f32>, interpolation: f32) -> f32 {
    range.start + (range.end - range.start) * interpolation
}

fn displacement_from_angle(angle: UnitComplex<f32>, distance: f32) -> Vector2<f32> {
    vector![angle.re, angle.im] * distance
}
