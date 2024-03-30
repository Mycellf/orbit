use crate::{app::App, input::InputButton, projectile::Projectile, projectile::Rectangle};
use macroquad::prelude::*;
use nalgebra::{vector, Complex, Point2, UnitComplex, Vector2};
use std::num::NonZeroU8;

use crate::input::InputAxis;

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

pub struct ArmorRing {
    pub armor: Vec<Option<Armor>>,
    pub radius: f32,
    pub angle: f32,
    pub speed: f32,
}

impl ArmorRing {
    pub fn from_size(
        size: Vector2<f32>,
        health: u8,
        count: usize,
        radius: f32,
        speed: f32,
    ) -> Self {
        let health = NonZeroU8::new(health).unwrap();
        let armor = (0..count).map(|_| Some(Armor { size, health })).collect();
        let angle = 0.0;
        Self {
            armor,
            radius,
            angle,
            speed,
        }
    }

    pub fn draw_around(&self, position: Point2<f32>, color: Color) {
        let mut angle = self.angle;
        let increment = self.get_increment();

        for armor in &self.armor {
            if let Some(Armor { size, .. }) = armor {
                draw_rectangle_ex(
                    position.x + self.radius * angle.cos(),
                    position.y + self.radius * angle.sin(),
                    size.y,
                    size.x,
                    DrawRectangleParams {
                        offset: vec2(0.0, 0.5),
                        rotation: angle,
                        color,
                    },
                );
            }
            angle += increment;
        }
    }

    pub fn update_angle(&mut self, delta_seconds: f32) {
        use std::f32::consts::PI;
        self.angle += self.speed * delta_seconds;
        self.angle %= 2.0 * PI;
    }

    pub fn get_full_radius(&self) -> f32 {
        let max_height = (&self.armor)
            .into_iter()
            .filter_map(|&a| a)
            .map(|a| a.size.y)
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap_or(0.0);
        max_height + self.radius
    }

    pub fn get_increment(&self) -> f32 {
        use std::f32::consts::PI;
        (2.0 * PI) / self.armor.len() as f32
    }

    pub fn get_colliders(&self, position: Point2<f32>) -> Vec<Option<Rectangle>> {
        let increment = self.get_increment();
        (&self.armor)
            .into_iter()
            .enumerate()
            .map(|(i, armor)| match armor {
                Some(armor) => {
                    let angle = UnitComplex::new(increment * i as f32 + self.angle);
                    Some(Rectangle::from_dimensions(
                        position + angle * vector![self.radius, 0.0],
                        vector![armor.size.y, armor.size.x],
                        vector![0.0, 0.5],
                        angle,
                    ))
                }
                None => None,
            })
            .collect()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Armor {
    pub size: Vector2<f32>,
    pub health: NonZeroU8,
}

pub struct Center {
    pub size: Vector2<f32>,
    pub health: NonZeroU8,
    pub angle: f32,
    pub speed: f32,
}

impl Center {
    pub fn from_size(size: Vector2<f32>, health: u8, speed: f32) -> Self {
        let health = NonZeroU8::new(health).unwrap();
        let angle = 0.0;
        Self {
            size,
            health,
            angle,
            speed,
        }
    }

    pub fn draw_around(&self, position: Point2<f32>, color: Color) {
        draw_rectangle_ex(
            position.x,
            position.y,
            self.size.x,
            self.size.y,
            DrawRectangleParams {
                offset: vec2(0.5, 0.5),
                rotation: self.angle,
                color,
            },
        );
    }

    pub fn update_angle(&mut self, delta_seconds: f32) {
        use std::f32::consts::PI;
        self.angle += self.speed * delta_seconds;
        self.angle %= 2.0 * PI;
    }

    pub fn get_collider(&self, position: Point2<f32>) -> Rectangle {
        Rectangle::from_dimensions(
            position,
            self.size,
            vector![0.5, 0.5],
            UnitComplex::new(self.angle),
        )
    }
}

#[derive(Clone, Debug)]
pub enum Controller {
    Player {
        speed: f32,
        x_control: InputAxis,
        y_control: InputAxis,
        shoot_control: Vec<InputButton>,
        cooldown: f32,
        shooting_speed: f32,
    },
}

impl Controller {
    pub fn update(entity: &mut Entity, delta_seconds: f32, app: &mut App) -> Option<()> {
        let controller = entity.controller.as_mut()?;
        match controller {
            Self::Player {
                speed,
                x_control,
                y_control,
                shoot_control,
                cooldown,
                shooting_speed,
            } => {
                // Mouse aim
                let aim = app.mouse.position - entity.position;
                let aim = UnitComplex::from_complex(Complex::new(aim.x, aim.y));
                entity.aim = Some(aim);

                // Motion
                x_control.update_state();
                y_control.update_state();
                let input = vector![x_control.as_f32(), y_control.as_f32()];
                let input = if input.x == 0.0 {
                    input
                } else {
                    input.normalize()
                };
                entity.position += input * (*speed * delta_seconds);

                // Shooting
                let shoot_pressed = shoot_control.into_iter().any(|b| b.is_down());
                let nudged_aim = UnitComplex::new(
                    aim.angle() + rand::gen_range(-0.1, 0.1) * (*shooting_speed - 1.0),
                );

                if *cooldown > 0.0 {
                    *cooldown -= delta_seconds;
                }
                if *cooldown <= 0.0 && shoot_pressed {
                    *cooldown = 0.5 / *shooting_speed;
                    app.projectiles.push(Projectile::from_speed(
                        48.0,
                        50.0,
                        nudged_aim,
                        entity.position + displacement_from_angle(nudged_aim, entity.radius + 6.0),
                        vector![1.0, 4.0],
                        2.0,
                        Color::from_hex(0x0000ff),
                    ));
                }

                if shoot_pressed {
                    if *shooting_speed <= 2.0 {
                        *shooting_speed += delta_seconds * 0.5;
                    } else {
                        *shooting_speed = 2.0;
                    }
                } else if *shooting_speed > 1.0 {
                    *shooting_speed -= delta_seconds;
                } else {
                    *shooting_speed = 1.0;
                }

                // Sync Mouse
                use std::f32::consts::PI;
                app.mouse.center_angle = entity.center.angle;
                if let Some(ring) = entity.rings.get(0) {
                    app.mouse.ring_angle = ring.angle - PI * 3.0 / 4.0;
                    app.mouse.set_active_from_ring(ring);
                } else {
                    app.mouse.active_corners = 0;
                }
                app.mouse.radius = (*shooting_speed - 1.0)
                    * (length(entity.position - app.mouse.position) - entity.radius - 6.0)
                    * 0.125;
                app.mouse.radius = app.mouse.radius.max(0.0);
            }
        }
        Some(())
    }
}

fn displacement_from_angle(angle: UnitComplex<f32>, distance: f32) -> Vector2<f32> {
    vector![angle.re, angle.im] * distance
}

fn length(vector: Vector2<f32>) -> f32 {
    (vector.x * vector.x + vector.y * vector.y).sqrt()
}
