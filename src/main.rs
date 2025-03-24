use computer_controller::Weapon;
use controller::SightKind;
use macroquad::prelude::*;
use nalgebra::{point, vector};

pub mod app;

pub mod projectile;

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
    use std::f32::consts::TAU;

    let mut app = app::App::from_ups(120.0);

    // Blue player entity
    app.entities.insert(entity::Entity::from_rings(
        point![-64.0, 0.0],
        Color::from_hex(0x0000ff),
        components::Center::from_size(vector![2.0, 2.0], 8, -TAU / 6.0),
        vec![
            components::ArmorRing::from_size(vector![4.0, 1.0], 4, 4, 3.5, TAU / 12.0),
            // *A gift to rustfmt to keep it from messing this code up*
        ],
        Some(controller::EntityController {
            targets: Vec::new(),
            motion: Some(controller::MotionController::Player(Default::default())),
            shooting: Some(controller::ShootingController::Player(Default::default())),
        }),
        controller::Team::Player,
    ));

    // Red entity
    app.entities.insert(entity::Entity::from_rings(
        point![48.0, 48.0],
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
                    speed: 10.0,
                    kind: computer_controller::ComputerMotionControllerKind::Circle {
                        distance: 50.0,
                        tangential_weight: -25.0,
                    },
                },
            )),
            shooting: Some(controller::ShootingController::Computer(
                computer_controller::ComputerShootingController {
                    weapon: Weapon {
                        initial_speed: 48.0,
                        speed_exponent: 50.0,
                        cooldown: 1.0,
                        projectiles_per_shot: 1,
                        projectile_angle: 0.0,
                        innacuracy: 0.0,
                        sight_kind: SightKind::Arrow,
                    },
                    aim: None,
                    cooldown: 0.0,
                    aiming_lead: 1.0,
                    lead_weight: 20.0,
                },
            )),
        }),
        controller::Team::Hostile,
    ));

    // Green entity
    app.entities.insert(entity::Entity::from_rings(
        point![48.0, -48.0],
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
                        distance: 48.0..64.0,
                    },
                },
            )),
            shooting: Some(controller::ShootingController::Computer(
                computer_controller::ComputerShootingController {
                    weapon: Weapon {
                        initial_speed: 48.0 * 5.0,
                        speed_exponent: 1.0 / 50.0,
                        cooldown: 2.0,
                        projectiles_per_shot: 9,
                        projectile_angle: TAU / 32.0 / 9.0,
                        innacuracy: 0.0,
                        sight_kind: SightKind::Cross,
                    },
                    aim: None,
                    cooldown: 0.0,
                    aiming_lead: 0.5,
                    lead_weight: 10.0,
                },
            )),
        }),
        controller::Team::Neutral,
    ));

    macroquad::input::show_mouse(false);

    let mut fullscreen = START_IN_FULLSCREEN;

    loop {
        if macroquad::input::is_key_pressed(KeyCode::F11) {
            fullscreen ^= true;
            set_fullscreen(fullscreen);
        }

        app.update();
        app.draw();

        next_frame().await;
    }
}
