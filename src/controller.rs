use crate::{
    app::App,
    computer_controller::{ComputerMotionController, ComputerShootingController},
    entity::Entity,
    player_controller::{PlayerMotionController, PlayerShootingController},
    util,
};
use nalgebra::UnitComplex;
use thunderdome::Index;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Team {
    Player,
    Neutral,
    Hostile,
}

#[derive(Clone, Debug)]
pub struct EntityController {
    pub targets: Vec<Index>,
    pub motion: Option<MotionController>,
    pub shooting: Option<ShootingController>,
}

impl EntityController {
    pub const AGGRO_DISTANCE: f32 = 100.0;

    pub fn update(entity: &mut Entity, index: Index, delta_seconds: f32, app: &mut App) {
        let entity_unsafe_borrow = unsafe { &mut *(entity as *mut Entity) }; // causes UB if safety rules are broken

        let controller = if let Some(controller) = entity.controller.as_mut() {
            controller
        } else {
            return;
        };

        let entity = entity_unsafe_borrow;

        if entity.team == Team::Hostile {
            for (other_index, other_entity) in &app.entities {
                if index != other_index
                    && other_entity.team == Team::Player
                    && util::length_squared(entity.position - other_entity.position)
                        < Self::AGGRO_DISTANCE.powi(2)
                {
                    controller.alert(other_index);
                }
            }
        }

        let mut i = 0;
        while i < controller.targets.len() {
            if app.entities.contains(controller.targets[i]) {
                i += 1;
            } else {
                controller.targets.remove(i);
            }
        }

        let targets = &mut controller.targets;

        if let Some(motion) = controller.motion.as_mut() {
            match motion {
                MotionController::Player(controller) => {
                    controller.update(entity);
                }
                MotionController::Computer(controller) => controller.update(entity, targets, app),
            }
        }

        if let Some(shooting) = controller.shooting.as_mut() {
            match shooting {
                ShootingController::Player(control) => {
                    control.update(index, entity, delta_seconds, app);
                }
                ShootingController::Computer(control) => {
                    control.update(index, entity, targets, delta_seconds, app)
                }
            }
        }
    }

    pub fn alert(&mut self, sender: Index) {
        if !self.targets.contains(&sender) {
            self.targets.push(sender);
        }
    }
}

#[derive(Clone, Debug)]
pub enum MotionController {
    Player(PlayerMotionController),
    Computer(ComputerMotionController),
}

#[derive(Clone, Debug)]
pub enum ShootingController {
    Player(PlayerShootingController),
    Computer(ComputerShootingController),
}

impl ShootingController {
    pub fn aim(&self) -> Option<(UnitComplex<f32>, f32, SightKind)> {
        match self {
            Self::Player(controller) => Some((
                controller.aim,
                controller.cooldown / controller.max_cooldown(),
                SightKind::Arrow,
            )),
            Self::Computer(controller) => controller.aim(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SightKind {
    Arrow,
    Cross,
}
