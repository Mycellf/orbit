use crate::entity::Entity;
use macroquad::prelude::*;
use std::time::Instant;

pub struct App {
    pub timestep_length: f32,
    pub update_time: f32,
    pub last_frame: Instant,
    pub camera: Camera2D,
    pub entities: Vec<Entity>,
}

impl App {
    pub fn from_ups(updates_per_second: f32) -> Self {
        let timestep_length = 1.0 / updates_per_second;
        let update_time = 0.0;
        let last_frame = Instant::now();
        let camera = Camera2D {
            zoom: Vec2::splat(1.0 / 48.0),
            ..Default::default()
        };
        let entities = Vec::new();
        Self {
            timestep_length,
            update_time,
            last_frame,
            camera,
            entities,
        }
    }

    pub fn draw(&mut self) {
        clear_background(BLACK);
        update_camera(&mut self.camera);

        for entity in &self.entities {
            entity.draw();
        }
    }

    pub fn update(&mut self) {
        self.update_time += self.last_frame.elapsed().as_secs_f32();
        self.last_frame = Instant::now();

        let updates = (self.update_time / self.timestep_length) as usize;
        for _ in 0..updates.min(5) {
            self.run_timestep();
        }

        self.update_time %= self.timestep_length;
    }

    pub fn run_timestep(&mut self) {
        for entity in &mut self.entities {
            entity.update(self.timestep_length);
        }
    }
}

fn update_camera(camera: &mut Camera2D) {
    camera.zoom.x = camera.zoom.y * screen_height() / screen_width();
    set_camera(camera);
}
