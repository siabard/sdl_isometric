use crate::components::*;
use crate::entities::*;
use crate::texture_manager::*;
use crate::timer::{Timer, TimerResult};
use crate::tween::*;
use crate::*;

use sdl2::render::WindowCanvas;
use std::collections::HashMap;

use uuid::Uuid;

/// 기본 엔터티
#[derive(Clone, Debug)]
pub struct Entity {
    pub type_: EntityType,
    pub id: Uuid,
    pub hitbox: Option<HitboxComponent>,
    pub animation: HashMap<Direction, AnimationComponent>,
    pub movement: Option<MovementComponent>,
    pub attack: Option<AttackComponent>,
    pub alive: bool,
    pub skill: HashMap<String, Timer>,
    //pub timer: Option<crate::timer::Timer>,
    //pub timer_result: Option<crate::timer::TimerResult>,
}

impl Entity {
    pub fn new(type_: EntityType) -> Entity {
        Entity {
            id: Uuid::new_v4(),
            type_,
            hitbox: None,
            movement: None,
            animation: HashMap::new(),
            attack: Some(AttackComponent::new()),
            alive: true,
            skill: HashMap::new(),
        }
    }

    pub fn set_hitbox(&mut self, x: f64, y: f64, hx: f64, hy: f64, w: f64, h: f64) {
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

    pub fn set_attack(&mut self) {
        self.attack = Some(AttackComponent::new());
    }

    pub fn get_predict_y(&self, dt: f64) -> f64 {
        self.movement.as_ref().unwrap().get_predict_y(dt)
    }

    pub fn get_predict_x(&self, dt: f64) -> f64 {
        self.movement.as_ref().unwrap().get_predict_x(dt)
    }

    pub fn update_predict(&mut self, dt: f64) {
        if let Some(movement) = self.movement.as_mut() {
            movement.update_predict(dt);

            if let Some(hitbox) = self.hitbox.as_mut() {
                hitbox.update(dt, movement.get_predict_x(dt), movement.get_predict_y(dt));
            }
        }
    }

    /// 기존에 값이 없으면 신규로 값을 넣는다.
    pub fn insert_timer(&mut self, s: String, t: Timer) {
        // 기존에 값이 없는 것만 넣는다.
        if !self.skill.contains_key(&s) {
            self.skill.insert(s, t);
        }
    }

    pub fn update_timer(&mut self, dt: f64) -> Vec<Option<TimerResult>> {
        // 타이머 처리
        let new_skill: Vec<(String, Timer)> = self
            .skill
            .clone()
            .into_iter()
            .map(|(s, mut v)| {
                if v.d >= v.t {
                    v.t += dt;
                    let t_after = tween::linear(v.t, v.b, v.c, v.d);
                }
                (s, v)
            })
            .collect();
        for (s, ts) in new_skill {
            self.skill.insert(s, ts);
        }

        let timer_result: Vec<Option<TimerResult>> = self
            .skill
            .clone()
            .into_iter()
            .filter(|(_, v)| v.t >= v.d)
            .map(|(s, v)| v.result)
            .collect();

        let remain_timer_result: Vec<(String, Timer)> = self
            .skill
            .clone()
            .into_iter()
            .filter(|(_, v)| v.t < v.d)
            .collect();

        self.skill = HashMap::new();
        for (s, ts) in remain_timer_result {
            self.skill.insert(s, ts);
        }

        timer_result
    }

    pub fn update(&mut self, dt: f64) {
        if let Some(movement) = self.movement.as_mut() {
            let direction = facing_to_direction(movement.get_facing());
            if let Some(animation) = self.animation.get_mut(&direction) {
                movement.update(dt);

                animation.x = movement.get_pos_x();
                animation.y = movement.get_pos_y();
                animation.update(dt);
            }

            if let Some(hitbox) = self.hitbox.as_mut() {
                hitbox.update(dt, movement.get_pos_x(), movement.get_pos_y());
            }

            if let Some(attack) = self.attack.as_mut() {
                attack.update(dt);
            }
        }
    }

    pub fn render(
        &self,
        canvas: &mut WindowCanvas,
        camera: &Rect,
        texture_manager: Option<&TextureManager>,
    ) {
        if let Some(movement) = self.movement.as_ref() {
            let direction = facing_to_direction(movement.get_facing());
            if let Some(animation) = self.animation.get(&direction) {
                if let Some(attack) = self.attack.as_ref() {
                    attack.render(canvas, camera, animation);
                }
                animation.render(canvas, camera, texture_manager.unwrap());
            }

            if let Some(hitbox) = self.hitbox.as_ref() {
                hitbox.render(canvas, camera);
            }
        }
    }
}
