use crate::{
    app::App,
    entity::Entity,
    input::{InputAxis, InputButton},
    projectile::Projectile,
};
use macroquad::{
    color::Color,
    input::{KeyCode, MouseButton},
    rand,
};
use nalgebra::{vector, Complex, Point2, UnitComplex, Vector2};
use std::ops::Range;
use thunderdome::Index;

#[derive(Clone, Debug)]
pub struct PlayerMotionController {
    pub x_control: InputAxis,
    pub y_control: InputAxis,
    pub speed: f32,
}

impl PlayerMotionController {
    pub fn update(&mut self, entity: &mut Entity) {
        self.x_control.update_state();
        self.y_control.update_state();
        let input = vector![self.x_control.as_f32(), self.y_control.as_f32()];
        let input = if input.x == 0.0 {
            input
        } else {
            input.normalize()
        };
        entity.velocity = input * self.speed;
    }
}

impl Default for PlayerMotionController {
    fn default() -> Self {
        Self {
            x_control: InputAxis::from_inputs(
                vec![KeyCode::D.into(), KeyCode::Right.into()],
                vec![KeyCode::A.into(), KeyCode::Left.into()],
            ),
            y_control: InputAxis::from_inputs(
                vec![KeyCode::S.into(), KeyCode::Down.into()],
                vec![KeyCode::W.into(), KeyCode::Up.into()],
            ),
            speed: 36.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlayerShootingController {
    pub shoot_control: Vec<InputButton>,
    pub precise_shoot_control: Vec<InputButton>,
    pub cooldown: f32,
    pub state: f32,
    pub speed: Range<f32>,
    pub precision: Range<f32>,
    pub delay: Range<f32>,
    pub aim: UnitComplex<f32>,
}

impl PlayerShootingController {
    pub fn update(&mut self, index: Index, entity: &mut Entity, delta_seconds: f32, app: &mut App) {
        let shoot_input = self.shoot_control.iter().any(|b| b.is_down());
        let precise_shoot_input = self.precise_shoot_control.iter().any(|b| b.is_down());

        let input = shoot_input || precise_shoot_input;
        let accelerate = shoot_input;

        let aim = app.mouse.position - entity.position;
        let aim = UnitComplex::from_complex(Complex::new(aim.x, aim.y));
        self.aim = aim;

        self.cooldown = (self.cooldown - delta_seconds).max(0.0);
        if input && self.cooldown <= 0.0 {
            self.cooldown = self.max_cooldown();
            let nudged_aim = UnitComplex::new(
                aim.angle() + rand::gen_range(-1.0, 1.0) * lerp(&self.precision, self.state),
            );
            insert_projectile(
                app,
                nudged_aim,
                entity.position,
                entity.radius + 4.0,
                entity.color,
                index,
            );
        }

        self.state += delta_seconds
            / if accelerate {
                self.delay.start
            } else {
                -self.delay.end
            };
        self.state = self.state.clamp(0.0, 1.0);

        // Sync mouse
        use std::f32::consts::TAU;
        app.mouse.center_angle = entity.center.angle;
        app.mouse.center_effect = entity.center.hit_effect;
        if let Some(ring) = entity.rings.get(0) {
            app.mouse.ring_angle = ring.angle - TAU * 3.0 / 8.0;
            app.mouse.set_effects_from_ring(ring);
        } else {
            app.mouse.ring_angle = entity.center.angle * -0.5 - (TAU * 3.0 / 8.0);
            app.mouse.set_effects_from_empty_ring();
        }
        app.mouse.radius = self.state * (length(entity.position - app.mouse.position)) * 0.125;
        app.mouse.radius = app.mouse.radius.max(0.0);
        app.mouse.color = entity.color;
        app.mouse.size_boost = (self.cooldown * 1.0 * u16::MAX as f32) as u16;
    }

    pub fn max_cooldown(&self) -> f32 {
        lerp(&self.speed, self.state)
    }
}

impl Default for PlayerShootingController {
    fn default() -> Self {
        Self {
            shoot_control: vec![
                InputButton::Keyboard(KeyCode::Space),
                InputButton::Mouse(MouseButton::Left),
            ],
            precise_shoot_control: vec![
                InputButton::Keyboard(KeyCode::RightAlt),
                InputButton::Keyboard(KeyCode::LeftAlt),
                InputButton::Mouse(MouseButton::Right),
            ],
            cooldown: 0.0,
            state: 0.0,
            speed: 0.5..0.25,
            precision: 0.0..0.15,
            delay: 1.0..2.0,
            aim: Default::default(),
        }
    }
}

fn insert_projectile(
    app: &mut App,
    aim: UnitComplex<f32>,
    position: Point2<f32>,
    offset_radius: f32,
    color: Color,
    sender: Index,
) {
    app.projectiles.insert(Projectile::from_speed(
        48.0,
        50.0,
        aim,
        position + displacement_from_angle(aim, offset_radius),
        vector![1.0, 4.0],
        1.0,
        color,
        sender,
    ));
}

fn length(vector: Vector2<f32>) -> f32 {
    length_squared(vector).sqrt()
}

fn length_squared(vector: Vector2<f32>) -> f32 {
    vector.x.powi(2) + vector.y.powi(2)
}

fn lerp(range: &Range<f32>, interpolation: f32) -> f32 {
    range.start + (range.end - range.start) * interpolation
}

fn displacement_from_angle(angle: UnitComplex<f32>, distance: f32) -> Vector2<f32> {
    vector![angle.re, angle.im] * distance
}
