use crate::{
    app::App,
    components::{ArmorRing, Center},
    controller::Controller,
};
use macroquad::prelude::*;
use nalgebra::{Point2, UnitComplex};

#[derive(Clone, Debug)]
pub struct Entity {
    pub rings: Vec<ArmorRing>,
    pub center: Center,
    pub position: Point2<f32>,
    pub aim: Option<UnitComplex<f32>>,
    pub radius: f32,
    pub color: Color,
    pub controller: Option<Controller>,
}

impl Entity {
    pub fn from_rings(
        position: Point2<f32>,
        color: Color,
        center: Center,
        rings: Vec<ArmorRing>,
        controller: Option<Controller>,
    ) -> Self {
        let aim = None;
        let radius = Self::get_radius_squared(&rings, &center).sqrt();
        Self {
            rings,
            center,
            position,
            aim,
            radius,
            color,
            controller,
        }
    }

    pub fn draw(&self) {
        use std::f32::consts::PI;

        self.center.draw_around(self.position, WHITE);
        for ring in &*self.rings {
            ring.draw_around(self.position, self.color);
        }

        if let Some(aim) = self.aim {
            let radius = self.radius + 4.0;

            draw_rectangle_ex(
                self.position.x + radius * aim.re,
                self.position.y + radius * aim.im,
                2.0,
                0.75,
                DrawRectangleParams {
                    offset: vec2(1.0, 0.0),
                    rotation: aim.angle() + PI / 4.0,
                    color: self.color,
                },
            );
            draw_rectangle_ex(
                self.position.x + radius * aim.re,
                self.position.y + radius * aim.im,
                0.75,
                2.0,
                DrawRectangleParams {
                    offset: vec2(1.0, 0.0),
                    rotation: aim.angle() + PI / 4.0,
                    color: self.color,
                },
            );
        }
    }

    pub fn update(&mut self, delta_seconds: f32, app: &mut App) {
        self.center.update(delta_seconds);
        for ring in &mut *self.rings {
            ring.update(delta_seconds);
        }

        Controller::update(self, delta_seconds, app);
    }

    /// Returning `None` indicates a request for deletion.
    pub fn check_deletion(&mut self) -> Option<()> {
        if self.center.armor.is_none() {
            return None;
        }

        let mut radius_squared = self.center.get_radius_squared();
        for i in (0..self.rings.len()).rev() {
            let ring = &mut self.rings[i];
            match ring.get_full_radius_squared() {
                Some(ring_radius) => {
                    radius_squared = radius_squared.max(ring_radius);
                }
                None => {
                    self.rings.swap_remove(i);
                }
            }
        }
        self.radius = radius_squared.sqrt();

        Some(())
    }

    pub fn get_full_radius(&self) -> f32 {
        Self::get_radius_squared(&self.rings, &self.center).sqrt()
    }

    fn get_radius_squared(rings: &Vec<ArmorRing>, center: &Center) -> f32 {
        rings
            .into_iter()
            .map(|r| r.get_full_radius_squared().unwrap_or(0.0))
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap_or_else(|| center.get_radius_squared())
    }
}
