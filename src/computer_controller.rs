use std::ops::Range;

use nalgebra::{Complex, Point2, UnitComplex, Vector2, vector};
use thunderdome::Index;

use crate::{app::App, entity::Entity, util};

#[derive(Clone, Debug)]
pub struct ComputerMotionController {
    pub speed: f32,
    pub kind: ComputerMotionControllerKind,
}

impl ComputerMotionController {
    pub fn update(&mut self, entity: &mut Entity, targets: &mut Vec<Index>, app: &mut App) {
        let Some((closest, displacement, distance_squared)) =
            closest_target(targets.iter(), entity.position, app)
        else {
            return;
        };

        let distance_to_target = distance_squared.sqrt();
        let direction = displacement / distance_to_target;

        match self.kind {
            ComputerMotionControllerKind::KeepDistance {
                distance:
                    Range {
                        start: min_distance,
                        end: max_distance,
                    },
            } => {
                if distance_to_target < min_distance {
                    entity.velocity = self.speed * -direction;
                } else if distance_to_target > max_distance {
                    entity.velocity = self.speed * direction;
                } else {
                    entity.velocity = vector![0.0, 0.0];
                }
            }
            ComputerMotionControllerKind::Circle {
                distance,
                tangential_weight,
            } => {
                let perpendicular = tangential_weight * vector![-direction.y, direction.x];
                let radial = direction * (distance_to_target - distance);

                entity.velocity = self.speed * (perpendicular + radial).normalize();
            }
            ComputerMotionControllerKind::Charge => {
                entity.velocity = self.speed * direction;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum ComputerMotionControllerKind {
    KeepDistance {
        distance: Range<f32>,
    },
    Circle {
        distance: f32,
        tangential_weight: f32,
    },
    Charge,
}

#[derive(Clone, Debug)]
pub struct ComputerShootingController {
    pub aim: Option<UnitComplex<f32>>,
    pub cooldown: f32,
}

impl ComputerShootingController {
    pub fn update(
        &mut self,
        index: Index,
        entity: &mut Entity,
        targets: &mut Vec<Index>,
        delta_seconds: f32,
        app: &mut App,
    ) {
        self.cooldown = (self.cooldown - delta_seconds).max(0.0);

        let Some((closest, displacement, distance_squared)) =
            closest_target(targets.iter(), entity.position, app)
        else {
            self.aim = None;

            return;
        };

        let target = &app.entities[closest];

        let aim = displacement;
        let aim = UnitComplex::from_complex(Complex::new(aim.x, aim.y));
        self.aim = Some(aim);

        if self.cooldown <= 0.0 {
            self.cooldown = 1.0;

            app.insert_projectile(
                aim,
                entity.position,
                entity.radius + 4.0,
                entity.color,
                entity.team,
                index,
            );
        }
    }

    pub fn aim(&self) -> Option<(UnitComplex<f32>, f32)> {
        if let Some(aim) = self.aim {
            Some((aim, self.cooldown))
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub enum ComputerAimKind {
    PointTowards,
}

#[derive(Clone, Debug)]
pub enum ComputerFiringKind {
    Always,
    WithinDistance { distance: Range<f32> },
}

pub struct Weapon {
    pub start_speed: f32,
    pub speed_exp: f32,
    pub max_cooldown: f32,
    pub projectiles_per_shot: usize,
    pub accuracy: f32,
}

pub fn closest_target<'a>(
    target_indecies: impl Iterator<Item = &'a Index>,
    position: Point2<f32>,
    app: &App,
) -> Option<(Index, Vector2<f32>, f32)> {
    target_indecies
        .filter_map(|&index| {
            let displacement = app.entities.get(index)?.position - position;
            Some((index, displacement, util::length_squared(displacement)))
        })
        .reduce(|a, b| if a.2 < b.2 { a } else { b })
}
