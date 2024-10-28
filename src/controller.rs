use crate::{
    app::App,
    entity::Entity,
    player_controller::{PlayerMotionController, PlayerShootingController},
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
    pub fn update(entity: &mut Entity, index: Index, delta_seconds: f32, app: &mut App) {
        let entity_unsafe_borrow = unsafe { &mut *(entity as *mut Entity) }; // causes UB if safety
                                                                             // rules are broken

        let controller = if let Some(controller) = entity.controller.as_mut() {
            controller
        } else {
            return;
        };

        let entity = entity_unsafe_borrow;

        if let Some(motion) = controller.motion.as_mut() {
            match motion {
                MotionController::Player(controller) => {
                    controller.update(entity);
                }
                MotionController::Computer {} => todo!(),
            }
        }

        if let Some(shooting) = controller.shooting.as_mut() {
            match shooting {
                ShootingController::Player(control) => {
                    control.update(index, entity, delta_seconds, app);
                }
                ShootingController::Computer {} => todo!(),
            }
        }
    }

    pub fn alert(&mut self, sender: Index) {}
}

#[derive(Clone, Debug)]
pub enum MotionController {
    Player(PlayerMotionController),
    Computer {},
}

#[derive(Clone, Debug)]
pub enum ShootingController {
    Player(PlayerShootingController),
    Computer {},
}

impl ShootingController {
    pub fn aim(&self) -> Option<(UnitComplex<f32>, f32)> {
        match self {
            Self::Player(controller) => Some((
                controller.aim,
                controller.cooldown / controller.max_cooldown(),
            )),
            _ => None,
        }
    }
}
