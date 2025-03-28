use std::f32::consts::TAU;

use computer_controller::Weapon;
use controller::SightKind;
use entity::Entity;
use macroquad::prelude::*;
use nalgebra::{Point2, point, vector};

pub mod app;

pub mod projectile;

pub mod camera;
pub mod components;
pub mod entity;
pub mod mouse_display;

pub mod computer_controller;
pub mod controller;
pub mod player_controller;

pub mod collision;
pub mod input;
pub mod util;

const START_IN_FULLSCREEN: bool = true;

fn window_conf() -> Conf {
    Conf {
        window_title: "Orbit".to_owned(),
        fullscreen: START_IN_FULLSCREEN,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = app::App::from_ups(120.0);

    // Blue player entity
    let player_index = app.entities.insert(entity::Entity::from_rings(
        point![-64.0, 0.0],
        Color::from_hex(0x0000ff),
        components::Center::from_size(vector![2.0, 2.0], 16, -TAU / 6.0),
        vec![
            components::ArmorRing::from_size(vector![4.0, 1.0], 8, 4, 3.5, TAU / 12.0),
            // *A gift to rustfmt to keep it from messing this code up*
        ],
        Some(controller::EntityController {
            targets: Vec::new(),
            motion: Some(controller::MotionController::Player(Default::default())),
            shooting: Some(controller::ShootingController::Player(Default::default())),
        }),
        controller::Team::Player,
    ));

    app.entities.insert(sniper(point![96.0, 16.0]));
    app.entities.insert(sniper(point![96.0, -16.0]));

    app.entities.insert(berzerker(point![64.0, 32.0]));
    app.entities.insert(berzerker(point![64.0, 0.0]));
    app.entities.insert(berzerker(point![64.0, -32.0]));

    app.entities.insert(turret_platform(point![128.0, 0.0]));

    app.entities.insert(neutral(point![-128.0, 0.0]));

    macroquad::input::show_mouse(false);

    let mut fullscreen = START_IN_FULLSCREEN;

    loop {
        if macroquad::input::is_key_pressed(KeyCode::F11) {
            fullscreen ^= true;
            set_fullscreen(fullscreen);
        }

        if macroquad::input::is_key_pressed(KeyCode::O) {
            for (_, entity) in &mut app.entities {
                let controller::Team::Hostile = entity.team else {
                    continue;
                };

                let Some(controller) = &mut entity.controller else {
                    continue;
                };

                controller.alert(player_index);
            }
        }

        app.update();
        app.draw();

        next_frame().await;
    }
}

// Strategy: zig-zag
pub fn sniper(position: Point2<f32>) -> Entity {
    entity::Entity::from_rings(
        position,
        Color::from_hex(0xff0000),
        components::Center::from_size(vector![2.0, 2.0], 8, TAU / 6.0),
        vec![
            components::ArmorRing::from_size(vector![4.0, 1.0], 4, 4, 3.5, -TAU / 12.0),
            components::ArmorRing::from_size(vector![2.0, 1.0], 2, 8, 6.0, TAU / 24.0),
        ],
        Some(controller::EntityController {
            targets: Vec::new(),
            motion: Some(controller::MotionController::Computer(
                computer_controller::ComputerMotionController {
                    speed: rand::gen_range(17.0, 19.0),
                    kind: computer_controller::ComputerMotionControllerKind::Circle {
                        distance: rand::gen_range(45.0, 50.0),
                        tangential_weight: rand::gen_range(-50.0, 50.0),
                    },
                },
            )),
            shooting: Some(controller::ShootingController::Computer(
                computer_controller::ComputerShootingController {
                    weapon: Weapon {
                        initial_speed: 48.0,
                        speed_exponent: 50.0,
                        cooldown: 2.0,
                        projectiles_per_shot: 1,
                        projectile_angle: 0.0,
                        projectile_spread: 0.0,
                        sight_kind: SightKind::Arrow,
                        sight_size: 1.0,
                    },
                    aim: None,
                    cooldown: 0.0,
                    aiming_lead: 1.0,
                    lead_weight: 5.0,
                },
            )),
        }),
        controller::Team::Hostile,
    )
}

// strategy: keep distance or circle around
pub fn berzerker(position: Point2<f32>) -> Entity {
    entity::Entity::from_rings(
        position,
        Color::from_hex(0xff0000),
        components::Center::from_size(vector![2.5, 2.0], 10, TAU / 3.0),
        vec![
            components::ArmorRing::from_size(vector![4.0, 1.0], 4, 4, 3.5, -TAU / 6.0),
            components::ArmorRing::from_size(vector![2.0, 0.5], 1, 8, 6.5, TAU / 12.0),
        ],
        Some(controller::EntityController {
            targets: Vec::new(),
            motion: Some(controller::MotionController::Computer(
                computer_controller::ComputerMotionController {
                    speed: rand::gen_range(17.0, 19.0),
                    kind: computer_controller::ComputerMotionControllerKind::KeepDistance {
                        distance: {
                            let start = rand::gen_range(0.0, 5.0);
                            let tolerance = rand::gen_range(0.0, 10.0);

                            start..start + tolerance
                        },
                    },
                },
            )),
            shooting: Some(controller::ShootingController::Computer(
                computer_controller::ComputerShootingController {
                    weapon: Weapon {
                        initial_speed: 48.0 * 5.0,
                        speed_exponent: 1.0 / 50.0,
                        cooldown: 1.0,
                        projectiles_per_shot: 2,
                        projectile_angle: 0.0,
                        projectile_spread: TAU / 32.0,
                        sight_kind: SightKind::Cross,
                        sight_size: 1.0,
                    },
                    aim: None,
                    cooldown: 0.0,
                    aiming_lead: 0.0,
                    lead_weight: 0.0,
                },
            )),
        }),
        controller::Team::Hostile,
    )
}

pub fn turret_platform(position: Point2<f32>) -> Entity {
    entity::Entity::from_rings(
        position,
        Color::from_hex(0xff0000),
        components::Center::from_size(vector![4.0, 4.0], 32, TAU / 6.0),
        vec![
            components::ArmorRing::from_size(vector![4.0, 1.0], 4, 4, 4.5, -TAU / 12.0),
            components::ArmorRing::from_size(vector![16.0, 2.0], 32, 2, 8.0, TAU / 24.0),
        ],
        Some(controller::EntityController {
            targets: Vec::new(),
            motion: None,
            shooting: Some(controller::ShootingController::Computer(
                computer_controller::ComputerShootingController {
                    weapon: Weapon {
                        initial_speed: 48.0 * 15.0,
                        speed_exponent: 1.0 / 50.0,
                        cooldown: 0.1,
                        projectiles_per_shot: 1,
                        projectile_angle: 0.0,
                        projectile_spread: TAU / 16.0,
                        sight_kind: SightKind::Arrow,
                        sight_size: 2.0,
                    },
                    aim: None,
                    cooldown: 0.0,
                    aiming_lead: 0.0,
                    lead_weight: 0.0,
                },
            )),
        }),
        controller::Team::Hostile,
    )
}

// strategy: keep distance or just don't aggro it
pub fn neutral(position: Point2<f32>) -> Entity {
    entity::Entity::from_rings(
        position,
        Color::from_hex(0x00ff00),
        components::Center::from_size(vector![2.0, 2.0], 8, TAU / 6.0),
        vec![
            components::ArmorRing::from_size(vector![2.0, 1.0], 2, 6, 3.5, TAU / 12.0),
            components::ArmorRing::from_size(vector![2.0, 1.0], 2, 12, 6.5, -TAU / 24.0),
            components::ArmorRing::from_size(vector![12.0, 2.0], 24, 3, 9.0, TAU / 48.0),
        ],
        Some(controller::EntityController {
            targets: Vec::new(),
            motion: Some(controller::MotionController::Computer(
                computer_controller::ComputerMotionController {
                    speed: 10.0,
                    kind: computer_controller::ComputerMotionControllerKind::KeepDistance {
                        distance: 32.0..40.0,
                    },
                },
            )),
            shooting: Some(controller::ShootingController::Computer(
                computer_controller::ComputerShootingController {
                    weapon: Weapon {
                        initial_speed: 48.0 * 5.0,
                        speed_exponent: 1.0 / 50.0,
                        cooldown: 3.0,
                        projectiles_per_shot: 9,
                        projectile_angle: TAU / 32.0 / 9.0,
                        projectile_spread: 0.0,
                        sight_kind: SightKind::Cross,
                        sight_size: 1.0,
                    },
                    aim: None,
                    cooldown: 0.0,
                    aiming_lead: 0.5,
                    lead_weight: 0.0,
                },
            )),
        }),
        controller::Team::Neutral,
    )
}
