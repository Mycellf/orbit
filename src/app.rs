use crate::{entity::Entity, mouse_display::MouseDisplay, projectile::Projectile, util};
use macroquad::prelude::*;
use nalgebra::{vector, Point2, UnitComplex};
use std::time::Instant;
use thunderdome::{Arena, Index};

pub struct App {
    pub timestep_length: f32,
    pub update_time: f32,
    pub last_frame: Instant,
    pub frame_time: f32,
    pub camera: Camera2D,
    pub entities: Arena<Entity>,
    pub projectiles: Arena<Projectile>,
    pub mouse: MouseDisplay,
}

impl App {
    pub const MAX_UPDATES_PER_FRAME: usize = 5;

    pub fn from_ups(updates_per_second: f32) -> Self {
        use std::f32::consts::TAU;
        let timestep_length = 1.0 / updates_per_second;
        let update_time = 0.0;
        let last_frame = Instant::now();
        let frame_time = 0.0;
        let camera = Camera2D {
            zoom: Vec2::splat(1.0 / 64.0),
            ..Default::default()
        };
        let entities = Arena::new();
        let projectiles = Arena::new();
        let mouse = MouseDisplay::from_speed(-TAU / 6.0, TAU / 12.0);
        Self {
            timestep_length,
            update_time,
            last_frame,
            frame_time,
            camera,
            entities,
            projectiles,
            mouse,
        }
    }

    pub fn draw(&mut self) {
        clear_background(BLACK);
        update_camera(&mut self.camera);

        let frame_time = self.frame_time.max(self.timestep_length);
        for (_, projectile) in &self.projectiles {
            projectile.draw(frame_time);
        }

        for (_, entity) in &self.entities {
            entity.draw();
        }

        self.mouse.draw();
    }

    pub fn update(&mut self) {
        self.frame_time = self.last_frame.elapsed().as_secs_f32();
        self.update_time += self.frame_time;
        self.last_frame = Instant::now();

        self.mouse.update_mouse_position(&self.camera);

        let updates = (self.update_time / self.timestep_length) as usize;
        for _ in 0..updates.min(Self::MAX_UPDATES_PER_FRAME) {
            self.run_timestep();
        }

        self.update_time %= self.timestep_length;
    }

    pub fn run_timestep(&mut self) {
        let app = unsafe { &mut *(self as *mut App) }; // Nececary due to the borrow checker, causes UB if
                                                       // safety rules are broken

        let mut to_remove = Vec::new();
        for (index, projectile) in &mut self.projectiles {
            if projectile.update(self.timestep_length, app).is_none() {
                to_remove.push(index);
            }
        }

        for index in to_remove {
            self.projectiles.remove(index);
        }

        for (index, entity) in &mut self.entities {
            entity.update(index, self.timestep_length, app);
        }
    }

    pub fn insert_projectile(
        &mut self,
        aim: UnitComplex<f32>,
        position: Point2<f32>,
        offset_radius: f32,
        color: Color,
        sender: Index,
    ) {
        self.projectiles.insert(Projectile::from_speed(
            48.0,
            50.0,
            aim,
            position + util::displacement_from_angle(aim, offset_radius),
            vector![1.0, 4.0],
            1.0,
            color,
            sender,
        ));
    }
}

fn update_camera(camera: &mut Camera2D) {
    camera.zoom.x = camera.zoom.y * screen_height() / screen_width();
    set_camera(camera);
}
