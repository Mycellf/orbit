use macroquad::prelude::*;
use nalgebra::{point, vector};

pub mod app;
pub mod collision;
pub mod components;
pub mod controller;
pub mod entity;
pub mod input;
pub mod mouse_display;
pub mod player_input;
pub mod projectile;

fn window_conf() -> Conf {
    Conf {
        window_title: "Orbit".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    use std::f32::consts::PI;
    let mut app = app::App::from_ups(120.0);
    app.entities.insert(entity::Entity::from_rings(
        point![-64.0, -16.0],
        Color::from_hex(0x0000ff),
        components::Center::from_size(vector![2.0, 2.0], 8, -PI / 3.0),
        vec![
            components::ArmorRing::from_size(vector![4.0, 1.0], 4, 4, 3.5, PI / 6.0),
            // *A gift to rustfmt to keep it from messing this code up*
        ],
        Some(controller::EntityController {
            motion: Some(controller::MotionController::Player(Default::default())),
            shooting: Some(controller::ShootingController::Player(Default::default())),
        }),
    ));
    app.entities.insert(entity::Entity::from_rings(
        point![64.0, 16.0],
        Color::from_hex(0xff0000),
        components::Center::from_size(vector![2.0, 2.0], 8, PI / 3.0),
        vec![
            components::ArmorRing::from_size(vector![4.0, 1.0], 4, 4, 3.5, -PI / 6.0),
            components::ArmorRing::from_size(vector![2.0, 1.0], 2, 8, 6.0, PI / 12.0),
        ],
        None,
    ));

    show_mouse(false);

    loop {
        app.update();
        app.draw();

        next_frame().await;
    }
}
