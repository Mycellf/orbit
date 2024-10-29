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

        let distance = distance_squared.sqrt();

        match self.kind {
            ComputerMotionControllerKind::KeepDistance {
                distance:
                    Range {
                        start: min_distance,
                        end: max_distance,
                    },
            } => {
                let direction = displacement / distance;

                if distance < min_distance {
                    entity.velocity = self.speed * -direction;
                } else if distance > max_distance {
                    entity.velocity = self.speed * direction;
                } else {
                    entity.velocity = vector![0.0, 0.0];
                }
            }
            ComputerMotionControllerKind::Circle { distance } => {}
            ComputerMotionControllerKind::Charge => {}
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
