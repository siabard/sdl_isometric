use crate::constant::*;
use crate::*;

use std::collections::HashMap;
use std::collections::HashSet;

use sdl2::render::Texture;
use sdl2::render::WindowCanvas;

pub fn facing_to_direction(facing: Vector2<i32>) -> Direction {
    if facing.0 > 0 {
        Direction::Right
    } else if facing.0 < 0 {
        Direction::Left
    } else if facing.1 < 0 {
        Direction::Up
    } else if facing.1 > 0 {
        Direction::Down
    } else {
        Direction::Stop
    }
}

/// 기본 엔터티
/// hitbox_component
/// movement_component
/// animation_component
pub struct Entity {
    pub type_: String,
    pub hitbox: Option<HitboxComponent>,
    pub animation: HashMap<Direction, AnimationComponent>,
    pub movement: Option<MovementComponent>,
}

impl Entity {
    pub fn new(type_: String) -> Entity {
        Entity {
            type_,
            hitbox: None,
            movement: None,
            animation: HashMap::new(),
        }
    }

    pub fn set_hitbox(&mut self, x: f64, y: f64, w: u32, h: u32) {
        self.hitbox = Some(HitboxComponent::new(x, y, w, h));
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

    pub fn update_predict(&mut self, dt: f64) {
        let movement = self.movement.as_mut().unwrap();
        let hitbox = self.hitbox.as_mut().unwrap();

        movement.update_predict(dt);
        hitbox.update(dt, movement.x, movement.y);
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

        animation.render(canvas, camera, texture);
    }
}

/// 충돌 좌표를 가지고 있는 부분
pub struct HitboxComponent {
    x: f64,
    y: f64,
    w: u32,
    h: u32,
}

impl HitboxComponent {
    pub fn new(x: f64, y: f64, w: u32, h: u32) -> HitboxComponent {
        HitboxComponent { x, y, w, h }
    }

    pub fn update(&mut self, _dt: f64, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(self.x as i32, self.y as i32, self.w, self.h)
    }

    pub fn is_collide(&mut self, hitbox: &HitboxComponent) -> HashSet<Direction> {
        detect_collision(&self.get_rect(), &hitbox.get_rect())
    }
}

/// 이동진행을 위한 부분
pub struct MovementComponent {
    pub x: f64,
    pub y: f64,
    px: f64,
    py: f64,
    pub facing: Vector2<i32>,
    velocity: Vector2<f64>,
    max_velocity: f64,
    accelaration: f64,
    decelaration: f64,
}

impl MovementComponent {
    pub fn new(
        x: f64,
        y: f64,
        facing: Vector2<i32>,
        velocity: Vector2<f64>,
        max_velocity: f64,
        accelaration: f64,
        decelaration: f64,
    ) -> MovementComponent {
        MovementComponent {
            x,
            y,
            px: x,
            py: y,
            facing,
            velocity,
            max_velocity,
            accelaration,
            decelaration,
        }
    }

    pub fn update_predict(&mut self, dt: f64) {
        // dt는 1000 밀리초(1초) 기준으로 한 프레임의 크기이다.
        // 해당 캐릭터는 dt 만큼 감속된 값으로 이동하게된다.
        // 이동 속도 벡터와 현재 속도를 비교하여 최대 이동폭을 구한다.

        if self.velocity.0 > 0. {
            // X 속도의 최대치 계산
            if self.velocity.0 > self.max_velocity {
                self.velocity.0 = self.max_velocity;
            }

            // X 감속
            self.velocity.0 -= self.decelaration * dt;

            // Bound condition
            if self.velocity.0 < 0. {
                self.velocity.0 = 0.
            }
        } else if self.velocity.0 < 0. {
            // X 속도의 최대치 계산
            if self.velocity.0 < -self.max_velocity {
                self.velocity.0 = -self.max_velocity;
            }

            // X 감속
            self.velocity.0 += self.decelaration * dt;

            // Bound condition
            if self.velocity.0 > 0. {
                self.velocity.0 = 0.
            }
        }

        if self.velocity.1 > 0. {
            // Y 속도의 최대치 계산
            if self.velocity.1 > self.max_velocity {
                self.velocity.1 = self.max_velocity;
            }

            // Y 감속
            self.velocity.1 -= self.decelaration * dt;

            // Bound condition
            if self.velocity.1 < 0. {
                self.velocity.1 = 0.
            }
        } else if self.velocity.1 < 0. {
            // Y 속도의 최대치 계산
            if self.velocity.1 < -self.max_velocity {
                self.velocity.1 = -self.max_velocity;
            }

            // Y 감속
            self.velocity.1 += self.decelaration * dt;

            // Bound condition
            if self.velocity.1 > 0. {
                self.velocity.1 = 0.
            }
        }

        self.px = self.x;
        self.py = self.y;

        // x, y 이동 단위 이동속도를 구했다면 px, py에 해당 값을 더한다.
        self.px += self.velocity.0 * dt;
        self.py += self.velocity.1 * dt;

        // x, y에 대한 Bound Condition
        if self.px < 0. {
            self.px = 0.;
        }
        if self.px > WORLD_WIDTH as f64 {
            self.px = WORLD_WIDTH as f64;
        }
        if self.py < 0. {
            self.py = 0.;
        }
        if self.py > WORLD_HEIGHT as f64 {
            self.py = WORLD_HEIGHT as f64;
        }
    }

    /// 해당 캐릭터의 x 속도를 0으로 리셋한다.
    /// 속도를 리셋하면서 px도 리셋한다.
    pub fn reset_velocity_x(&mut self) {
        self.velocity.0 = 0.0;
        self.px = self.x;
    }

    /// 해당 캐릭터의 y 속도를 0으로 리셋한다.
    /// 속도를 리셋하면서 py도 리셋한다.
    pub fn reset_velocity_y(&mut self) {
        self.velocity.1 = 0.0;
        self.py = self.y;
    }

    /// 캐릭터의 이동 벡터를 설정한다.
    pub fn move_forward(&mut self, direction: (f64, f64), dt: f64) {
        self.velocity.0 += direction.0 * self.accelaration * dt;
        self.velocity.1 += direction.1 * self.accelaration * dt;
    }

    /// 해당 캐릭터를 움직이게한다.
    pub fn update(&mut self, _dt: f64) {
        // 먼저 계산한 px, py 값을 새로운 x, y값으로 전환한다.

        self.x = self.px;
        self.y = self.py;
    }
}

/// 애니메이션을 위한 부분
pub struct AnimationComponent {
    pub x: f64,
    pub y: f64,
    w: u32,
    h: u32,
    frames: Vec<Rect>,
    frame: usize,
    max_frame: usize,
    timer: f64,
    span: f64,
    flip_h: bool,
    flip_v: bool,
}

impl AnimationComponent {
    pub fn new(
        x: f64,
        y: f64,
        w: u32,
        h: u32,
        frames: Vec<Rect>,
        frame: usize,
        max_frame: usize,
        span: f64,
        flip_h: bool,
        flip_v: bool,
    ) -> AnimationComponent {
        AnimationComponent {
            x,
            y,
            w,
            h,
            frames,
            frame,
            max_frame,
            timer: 0.0,
            span,
            flip_h,
            flip_v,
        }
    }

    pub fn update(&mut self, dt: f64) {
        // timer에 dt를 누적해서 span보다 커지면 한 프레임씩 증가한다.
        // 이렇게 하면 1초에 몇프레임 식으로 애니메이션을 조작할 수 있다.

        self.timer += dt;

        if self.timer > self.span {
            // 이동 속도가 있어야 프레임을 증가시킨다.

            self.frame += 1;
            if self.frame >= self.max_frame {
                self.frame = 0;
            }
            self.timer = 0.0;
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas, camera: &Rect, texture: &Texture) {
        let rect = Rect::new(
            transform_value(self.x as i32 - camera.x, WIDTH_RATIO),
            transform_value(self.y as i32 - camera.y, HEIGHT_RATIO),
            transform_value(self.w, WIDTH_RATIO),
            transform_value(self.h, WIDTH_RATIO),
        );
        let src = self.frames[self.frame as usize];
        canvas
            .copy_ex(
                texture,
                Some(src),
                Some(rect),
                0.,
                None,
                self.flip_h,
                self.flip_v,
            )
            .unwrap();
    }
}
