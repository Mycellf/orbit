use macroquad::prelude::*;
use nalgebra::{point, vector};

pub mod app;
pub mod entity;
pub mod input;

fn window_conf() -> Conf {
    Conf {
        window_title: "Orbit Strike".to_owned(),
        // fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    use std::f32::consts::PI;
    let mut app = app::App::from_ups(120.0);
    app.entities.push(entity::Entity::from_rings(
        point![0.0, 0.0],
        BLUE,
        entity::Center::from_size(vector![2.0, 2.0], 8, -PI / 6.0),
        vec![
            entity::ArmorRing::from_size(vector![4.0, 1.0], 2, 4, 3.5, PI / 6.0),
            //
        ],
        entity::Controller::Player { speed: 24.0 },
    ));

    loop {
        app.update();
        app.draw();

        next_frame().await;
    }
}
