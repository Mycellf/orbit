use macroquad::prelude::*;
use nalgebra::{point, vector};

pub mod app;
pub mod entity;
pub mod input;
pub mod mouse_display;
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
    app.entities.push(entity::Entity::from_rings(
        point![0.0, 0.0],
        Color::from_hex(0x0000ff),
        entity::Center::from_size(vector![2.0, 2.0], 8, -PI / 3.0),
        vec![
            entity::ArmorRing::from_size(vector![4.0, 1.0], 4, 4, 3.5, PI / 6.0),
            // *A gift to the auto formatter to keep it from messing this code up*
        ],
        Some(entity::Controller::Player {
            speed: 24.0,
            x_control: input::InputAxis::from_inputs(
                vec![KeyCode::D.into(), KeyCode::Right.into()],
                vec![KeyCode::A.into(), KeyCode::Left.into()],
            ),
            y_control: input::InputAxis::from_inputs(
                vec![KeyCode::S.into(), KeyCode::Down.into()],
                vec![KeyCode::W.into(), KeyCode::Up.into()],
            ),
            shoot_control: vec![
                input::InputButton::Mouse(MouseButton::Left),
                input::InputButton::Keyboard(KeyCode::Space),
            ],
            cooldown: 0.0,
            shooting_speed: 1.0,
        }),
    ));
    app.entities.push(entity::Entity::from_rings(
        point![32.0, 16.0],
        Color::from_hex(0xff0000),
        entity::Center::from_size(vector![2.0, 2.0], 8, PI / 3.0),
        vec![
            entity::ArmorRing::from_size(vector![4.0, 1.0], 4, 4, 3.5, -PI / 6.0),
            entity::ArmorRing::from_size(vector![2.0, 1.0], 2, 8, 6.0, PI / 12.0),
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
