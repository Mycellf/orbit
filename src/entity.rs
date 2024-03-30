use crate::{
    app::App,
    components::{ArmorRing, Center},
    controller::Controller,
};
use macroquad::prelude::*;
use nalgebra::{Point2, UnitComplex};

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
        let radius = Self::get_radius_of(&rings, &center);
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
        self.center.update_angle(delta_seconds);
        for ring in &mut *self.rings {
            ring.update_angle(delta_seconds);
        }

        Controller::update(self, delta_seconds, app);
    }

    pub fn get_full_radius(&self) -> f32 {
        Self::get_radius_of(&self.rings, &self.center)
    }

    fn get_radius_of(rings: &Vec<ArmorRing>, center: &Center) -> f32 {
        rings
            .into_iter()
            .map(|r| r.get_full_radius())
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap_or_else(|| center.size.x.max(center.size.y) / 2.0)
    }
}
