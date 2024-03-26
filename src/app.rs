use crate::entity::Entity;
use crate::mouse_display::MouseDisplay;
use macroquad::prelude::*;
use std::time::Instant;

pub struct App {
    pub timestep_length: f32,
    pub update_time: f32,
    pub last_frame: Instant,
    pub camera: Camera2D,
    pub entities: Vec<Entity>,
    pub mouse: MouseDisplay,
}

impl App {
    pub fn from_ups(updates_per_second: f32) -> Self {
        use std::f32::consts::PI;
        let timestep_length = 1.0 / updates_per_second;
        let update_time = 0.0;
        let last_frame = Instant::now();
        let camera = Camera2D {
            zoom: Vec2::splat(1.0 / 64.0),
            ..Default::default()
        };
        let entities = Vec::new();
        let mouse = MouseDisplay::from_speed(-PI / 3.0, PI / 6.0);
        Self {
            timestep_length,
            update_time,
            last_frame,
            camera,
            entities,
            mouse,
        }
    }

    pub fn draw(&mut self) {
        clear_background(BLACK);
        update_camera(&mut self.camera);

        for entity in &self.entities {
            entity.draw();
        }

        self.mouse.draw();
    }

    pub fn update(&mut self) {
        self.update_time += self.last_frame.elapsed().as_secs_f32();
        self.last_frame = Instant::now();

        self.mouse.update_mouse_position(&self.camera);

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

        self.mouse.update(self.timestep_length);
    }
}

fn update_camera(camera: &mut Camera2D) {
    camera.zoom.x = camera.zoom.y * screen_height() / screen_width();
    set_camera(camera);
}
