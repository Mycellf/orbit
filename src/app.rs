use crate::entity::Entity;
use macroquad::prelude::*;
use nalgebra::{point, Point2};
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

pub struct MouseDisplay {
    radius: f32,
    center_angle: f32,
    center_speed: f32,
    ring_angle: f32,
    ring_speed: f32,
    position: Point2<f32>,
}

impl MouseDisplay {
    pub fn from_speed(center_speed: f32, ring_speed: f32) -> Self {
        let radius = 0.0;
        let center_angle = 0.0;
        let ring_angle = 0.0;
        let position = point![0.0, 0.0];
        Self {
            radius,
            center_angle,
            center_speed,
            ring_angle,
            ring_speed,
            position,
        }
    }

    pub fn update_mouse_position(&mut self, camera: &Camera2D) {
        let position = mouse_position_local() / camera.zoom + camera.target;
        self.position = position.into();
    }

    pub fn update(&mut self, delta_seconds: f32) {
        use std::f32::consts::PI;
        self.center_angle += self.center_speed * delta_seconds;
        self.center_angle %= PI / 2.0;
        self.ring_angle += self.ring_speed * delta_seconds;
        self.ring_angle %= PI / 2.0;
    }

    pub fn draw(&self) {
        use std::f32::consts::{PI, SQRT_2};

        draw_rectangle_ex(
            self.position.x,
            self.position.y,
            1.0,
            1.0,
            DrawRectangleParams {
                offset: vec2(0.5, 0.5),
                rotation: self.center_angle,
                color: WHITE,
            },
        );

        let cos = (self.ring_angle + PI / 4.0).cos();
        let sin = (self.ring_angle + PI / 4.0).sin();
        let radius = self.radius + SQRT_2 / 2.0;
        for (x, y) in get_rotations_of(cos * radius, sin * radius) {
            draw_rectangle_ex(
                self.position.x + x,
                self.position.y + y,
                1.0,
                1.0,
                DrawRectangleParams {
                    offset: vec2(0.5, 0.5),
                    rotation: self.ring_angle,
                    color: BLUE,
                },
            );
        }
    }
}

/// Generates four clockwise rotations of x and y
fn get_rotations_of<T>(x: T, y: T) -> [(T, T); 4]
where
    T: std::ops::Neg<Output = T> + Copy,
{
    [(x, y), (y, -x), (-x, -y), (-y, x)]
}
