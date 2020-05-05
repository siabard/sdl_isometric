use crate::components::*;
use crate::entities::*;
use crate::*;

use sdl2::render::Texture;
use sdl2::render::WindowCanvas;
use std::collections::HashMap;

use uuid::Uuid;

/// 기본 엔터티
/// hitbox_component
/// movement_component
/// animation_component
#[derive(Clone, Debug)]
pub struct Entity {
    pub type_: EntityType,
    pub id: Uuid,
    pub hitbox: Option<HitboxComponent>,
    pub animation: HashMap<Direction, AnimationComponent>,
    pub movement: Option<MovementComponent>,
}

impl Entity {
    pub fn new(type_: EntityType) -> Entity {
        Entity {
            id: Uuid::new_v4(),
            type_,
            hitbox: None,
            movement: None,
            animation: HashMap::new(),
        }
    }

    pub fn set_hitbox(&mut self, x: f64, y: f64, hx: f64, hy: f64, w: u32, h: u32) {
        self.hitbox = Some(HitboxComponent::new(x, y, hx, hy, w, h));
    }

    pub fn set_movement(
        &mut self,
        x: f64,
        y: f64,
        facing: Vector2<i32>,
        velocity: Vector2<f64>,
        max_velocity: f64,
        accelaration: f64,
        decelaration: f64,
    ) {
        self.movement = Some(MovementComponent::new(
            x,
            y,
            facing,
            velocity,
            max_velocity,
            accelaration,
            decelaration,
        ));
    }

    pub fn get_predict_y(&self, dt: f64) -> f64 {
        self.movement.unwrap().get_predict_y(dt)
    }

    pub fn get_predict_x(&self, dt: f64) -> f64 {
        self.movement.unwrap().get_predict_x(dt)
    }

    pub fn update_predict(&mut self, dt: f64) {
        let movement = self.movement.as_mut().unwrap();
        let hitbox = self.hitbox.as_mut().unwrap();

        movement.update_predict(dt);
        hitbox.update(dt, movement.px, movement.py);
    }

    pub fn update(&mut self, dt: f64) {
        let movement = self.movement.as_mut().unwrap();
        let hitbox = self.hitbox.as_mut().unwrap();

        let direction = facing_to_direction(movement.facing);
        let animation = self.animation.get_mut(&direction).unwrap();

        movement.update(dt);

        animation.x = movement.x;
        animation.y = movement.y;

        animation.update(dt);

        hitbox.update(dt, movement.x, movement.y);
    }

    pub fn render(&self, canvas: &mut WindowCanvas, camera: &Rect, texture: &Texture) {
        let movement = self.movement.as_ref().unwrap();
        let direction = facing_to_direction(movement.facing);
        let animation = self.animation.get(&direction).unwrap();
        let hitbox = self.hitbox.as_ref().unwrap();

        animation.render(canvas, camera, texture);
        hitbox.render(canvas, camera);
    }
}
