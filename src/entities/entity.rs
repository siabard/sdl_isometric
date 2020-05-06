use crate::components::*;
use crate::entities::*;
use crate::*;

use sdl2::render::Texture;
use sdl2::render::WindowCanvas;
use std::collections::HashMap;

use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

/// 기본 엔터티
/// hitbox_component
/// movement_component -> 어떤 FnMut 를 호출하느냐에 맞추면 된다.
/// movement 에는 해당하는 특성을 넣어준다.
/// 이걸 SPECS로 하면 더 쉽겠지?
/// animation_component
#[derive(Clone)]
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
        self.movement.as_ref().unwrap().get_predict_y(dt)
    }

    pub fn get_predict_x(&self, dt: f64) -> f64 {
        self.movement.as_ref().unwrap().get_predict_x(dt)
    }

    pub fn update_predict(&mut self, dt: f64) {
        let movement = self.movement.as_mut().unwrap();
        let hitbox = self.hitbox.as_mut().unwrap();

        movement.update_predict(dt);
        hitbox.update(dt, movement.get_predict_x(dt), movement.get_predict_y(dt));
    }

    pub fn update(&mut self, dt: f64) {
        let movement = self.movement.as_mut().unwrap();
        let hitbox = self.hitbox.as_mut().unwrap();

        let direction = facing_to_direction(movement.get_facing());
        let animation = self.animation.get_mut(&direction).unwrap();

        movement.update(dt);

        animation.x = movement.get_pos_x();
        animation.y = movement.get_pos_y();

        animation.update(dt);

        hitbox.update(dt, movement.get_pos_x(), movement.get_pos_y());
    }

    pub fn render(&self, canvas: &mut WindowCanvas, camera: &Rect, texture: &Texture) {
        let movement = self.movement.as_ref().unwrap();
        let direction = facing_to_direction(movement.get_facing());
        let animation = self.animation.get(&direction).unwrap();
        let hitbox = self.hitbox.as_ref().unwrap();

        animation.render(canvas, camera, texture);
        hitbox.render(canvas, camera);
    }
}
