use macroquad::{camera::Camera2D, math::Vec2};
use nalgebra::vector;

use crate::input::{InputAxis, InputButton};

pub enum CameraControl {
    Manual {
        vertical: InputAxis,
        horizontal: InputAxis,
        boost: Vec<InputButton>,
        default_speed: f32,
        boost_speed: f32,
        normalized: bool,
    },
}

impl CameraControl {
    pub fn update_camera(&mut self, camera: &mut Camera2D, delta_seconds: f32) {
        match self {
            CameraControl::Manual {
                vertical,
                horizontal,
                boost,
                default_speed,
                boost_speed,
                normalized,
            } => {
                vertical.update_state();
                horizontal.update_state();

                let mut input_velocity = vector![horizontal.as_f32(), vertical.as_f32()];

                if *normalized && input_velocity != vector![0.0, 0.0] {
                    input_velocity.normalize_mut();
                }

                let speed = if boost.iter().any(|b| b.is_down()) {
                    *boost_speed
                } else {
                    *default_speed
                };

                let displacement = input_velocity * speed * delta_seconds;

                camera.target += Vec2::from(<[f32; 2]>::from(displacement));
            }
        }
    }
}
