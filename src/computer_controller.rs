use std::ops::Range;

use macroquad::rand;
use nalgebra::{Complex, Point2, UnitComplex, Vector2, vector};
use thunderdome::Index;

use crate::{app::App, controller::SightKind, entity::Entity, util};

#[derive(Clone, Debug)]
pub struct ComputerMotionController {
    pub speed: f32,
    pub kind: ComputerMotionControllerKind,
}

impl ComputerMotionController {
    pub fn update(&mut self, entity: &mut Entity, targets: &mut Vec<Index>, app: &mut App) {
        let Some((target_index, displacement, distance_squared)) =
            closest_target(targets.iter(), entity.position, app)
        else {
            entity.velocity = [0.0; 2].into();

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
    pub weapon: Weapon,
    pub aim: Option<UnitComplex<f32>>,
    pub cooldown: f32,
    /// 1.0 means use leading, 0.0 means ignore leading
    pub aiming_lead: f32,
    /// this is added to the muzzle distance when calculating target leading
    /// 10.0 typically leads to best performance
    pub lead_weight: f32,
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

        let Some((target_index, displacement, distance_squared)) =
            closest_target(targets.iter(), entity.position, app)
        else {
            self.aim = None;

            return;
        };

        let target = &app.entities[target_index];

        let lead_amount = if self.aiming_lead != 0.0 {
            let muzzle_length = entity.radius + 4.0;
            let muzzle_distance = distance_squared.sqrt() - muzzle_length;

            let expected_time_to_target =
                ((muzzle_distance + self.lead_weight) * 50.0f32.ln() / 48.0).log(50.0);

            expected_time_to_target * target.velocity * self.aiming_lead
        } else {
            [0.0; 2].into()
        };

        let aim = displacement + lead_amount;

        let aim = UnitComplex::from_complex(Complex::new(aim.x, aim.y));
        self.aim = Some(aim);

        if self.cooldown <= 0.0 {
            self.cooldown = self.weapon.cooldown;

            let aim_angle = aim.angle();
            let projectile_sweep =
                (self.weapon.projectiles_per_shot - 1) as f32 * self.weapon.projectile_angle;

            let start_angle = aim_angle - projectile_sweep / 2.0;

            for i in 0..self.weapon.projectiles_per_shot {
                let angle = start_angle + i as f32 * self.weapon.projectile_angle;

                let nudged_aim =
                    UnitComplex::new(angle + rand::gen_range(-1.0, 1.0) * self.weapon.innacuracy);

                app.insert_projectile(
                    self.weapon.initial_speed,
                    self.weapon.speed_exponent,
                    nudged_aim,
                    entity.position,
                    entity.radius + 4.0,
                    entity.color,
                    entity.team,
                    index,
                );
            }
        }
    }

    pub fn aim(&self) -> Option<(UnitComplex<f32>, f32, SightKind)> {
        if let Some(aim) = self.aim {
            Some((aim, self.cooldown, self.weapon.sight_kind))
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

#[derive(Clone, Debug)]
pub struct Weapon {
    pub initial_speed: f32,
    pub speed_exponent: f32,
    pub cooldown: f32,
    pub projectiles_per_shot: usize,
    pub projectile_angle: f32,
    pub innacuracy: f32,
    pub sight_kind: SightKind,
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
