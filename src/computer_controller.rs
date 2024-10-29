use std::ops::Range;

use nalgebra::vector;
use thunderdome::Index;

use crate::{app::App, entity::Entity, util};

#[derive(Clone, Debug)]
pub struct ComputerMotionController {
    pub speed: f32,
    pub kind: ComputerMotionControllerKind,
}

impl ComputerMotionController {
    pub fn update(&mut self, entity: &mut Entity, targets: &mut Vec<Index>, app: &mut App) {
        let Some((closest, displacement, distance_squared)) = (targets.iter())
            .map(|&index| {
                let displacement = app.entities[index].position - entity.position;
                (index, displacement, util::length_squared(displacement))
            })
            .reduce(|a, b| if a.2 < b.2 { a } else { b })
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
            ComputerMotionControllerKind::Circle { distance } => {
                if distance_to_target < distance * 0.5 {
                    entity.velocity = self.speed * -direction;
                } else {
                    let perpendicular = vector![-direction.y, direction.x];
                    let target = distance * perpendicular + 0.1 * direction + displacement;

                    entity.velocity = self.speed * target.normalize();
                }
            }
            ComputerMotionControllerKind::Charge => {
                entity.velocity = self.speed * direction;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum ComputerMotionControllerKind {
    KeepDistance { distance: Range<f32> },
    Circle { distance: f32 },
    Charge,
}

#[derive(Clone, Debug)]
pub struct ComputerShootingController {}
