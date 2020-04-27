use crate::constant::*;
use crate::*;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::WindowCanvas;
use std::collections::HashMap;
/// Animation 을 수행하는 내역
/// 개별 캐릭터는 하나의 UnitCharacter 이다.
#[derive(Clone, PartialEq)]
pub struct UnitCharacter {
    hitbox: Option<Rect>,
    animation: HashMap<Direction, (Vec<Rect>, bool, bool)>,
    pub x: f32,
    pub y: f32,
    pub w: u32,
    pub h: u32,
    pub px: f32,
    pub py: f32,
    frame: u32,
    max_frame: u32,
    timer: f64,
    pub span: f64,            // 한 프레임에 필요한 시간
    decelaration: f32,        // 감속정도
    accelaration: f32,        // 가속정도
    pub velocity: (f32, f32), // 속도
    max_velocity: f32,        // 이론상 최대속도
    pub direction: Direction, // 바라보는 방향
}

impl UnitCharacter {
    /// 개별 캐릭터를 등록한다.
    pub fn new(
        w: u32,
        h: u32,
        max_frame: u32,
        max_velocity: f32,
        accelaration: f32,
        decelaration: f32,
    ) -> UnitCharacter {
        UnitCharacter {
            hitbox: None,
            animation: HashMap::new(),
            x: 0.,
            y: 0.,
            px: 0.,
            py: 0.,
            w: w,
            h: h,
            frame: 0,
            max_frame: max_frame,
            timer: 0.0f64,
            span: 1.0 / 4.0, // 0.25초마당 한 프레임, 즉 초당 4 프레임을 움직인다.
            decelaration,
            accelaration,
            velocity: (0., 0.),
            max_velocity,
            direction: Direction::Left,
        }
    }

    /// hitbox를 등록한다.
    /// x : 좌상단부터 떨어진 가로위치
    /// y : 좌상단부터 떨어진 세로위치
    pub fn set_hitbox(&mut self, x: i32, y: i32, w: u32, h: u32) {
        self.hitbox = Some(Rect::new(x, y, w, h));
    }

    /// 캐릭터의 위치에 맞춘 정확한 예상충돌영역을 가져온다.
    pub fn get_hitbox_predict(&self) -> Option<Rect> {
        match self.hitbox {
            Some(hitbox) => {
                let hx = self.px as i32 + hitbox.x;
                let hy = self.py as i32 + hitbox.y;
                Some(Rect::new(hx, hy, hitbox.width(), hitbox.height()))
            }
            None => None,
        }
    }

    /// 캐릭터의 위치에 맞춘 정확한 충돌영역을 가져온다.
    pub fn get_hitbox(&self) -> Option<Rect> {
        match self.hitbox {
            Some(hitbox) => {
                let hx = self.x as i32 + hitbox.x;
                let hy = self.y as i32 + hitbox.y;
                Some(Rect::new(hx, hy, hitbox.width(), hitbox.height()))
            }
            None => None,
        }
    }

    /// animation을 등록한다.
    pub fn set_animation(
        &mut self,
        direction: Direction,
        frames: Vec<Rect>,
        flip_h: bool,
        flip_v: bool,
    ) {
        self.animation.insert(direction, (frames, flip_h, flip_v));
    }

    /// 캐릭터의 이동 벡터를 설정한다.
    pub fn move_forward(&mut self, direction: (f32, f32), dt: f64) {
        self.velocity.0 += (direction.0 as f64 * self.accelaration as f64 * dt) as f32;
        self.velocity.1 += (direction.1 as f64 * self.accelaration as f64 * dt) as f32;
    }

    /// 해당 캐릭터의 이동 위치를 예상한다.
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
            self.velocity.0 -= (self.decelaration as f64 * dt) as f32;

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
            self.velocity.0 += (self.decelaration as f64 * dt) as f32;

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
            self.velocity.1 -= (self.decelaration as f64 * dt) as f32;

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
            self.velocity.1 += (self.decelaration as f64 * dt) as f32;

            // Bound condition
            if self.velocity.1 > 0. {
                self.velocity.1 = 0.
            }
        }

        self.px = self.x;
        self.py = self.y;

        // x, y 이동 단위 이동속도를 구했다면 px, py에 해당 값을 더한다.
        self.px = self.px + self.velocity.0 * dt as f32;
        self.py = self.py + self.velocity.1 * dt as f32;

        // x, y에 대한 Bound Condition
        if self.px < 0. {
            self.px = 0.;
        }
        if self.px > WORLD_WIDTH as f32 {
            self.px = WORLD_WIDTH as f32;
        }
        if self.py < 0. {
            self.py = 0.;
        }
        if self.py > WORLD_HEIGHT as f32 {
            self.py = WORLD_HEIGHT as f32;
        }
    }

    /// 해당 캐릭터의 x 속도를 0으로 리셋한다.
    /// 속도를 리셋하면서 x도 px로 리셋한다.
    pub fn reset_velocity_x(&mut self) {
        self.velocity.0 = 0.0;
        self.px = self.x;
    }

    /// 해당 캐릭터의 y 속도를 0으로 리셋한다.
    /// 속도를 리셋하면서 y도 py로 리셋한다.
    pub fn reset_velocity_y(&mut self) {
        self.velocity.1 = 0.0;
        self.py = self.y;
    }

    /// 해당 캐릭터를 움직이게한다.
    pub fn update(&mut self, dt: f64) {
        // 먼저 계산한 px, py 값을 새로운 x, y값으로 전환한다.

        self.x = self.px;
        self.y = self.py;

        // 캐릭터의 방향을 이동 속도에 맞추어 정한다.
        /*
        if self.velocity.0.abs() > self.velocity.1.abs() {
            if self.velocity.0 > 0. {
                self.direction = Direction::Right;
            } else {
                self.direction = Direction::Left;
            }
        } else {
            if self.velocity.1 > 0. {
                self.direction = Direction::Down;
            } else {
                self.direction = Direction::Up;
            }
        }
        */

        // timer에 dt를 누적해서 span보다 커지면 한 프레임씩 증가한다.
        // 이렇게 하면 1초에 몇프레임 식으로 애니메이션을 조작할 수 있다.

        self.timer += dt;

        if self.timer > self.span {
            // 이동 속도가 있어야 프레임을 증가시킨다.
            if self.velocity.0.abs() > 0. || self.velocity.1.abs() > 0. {
                self.frame = self.frame + 1;
                if self.frame >= self.max_frame {
                    self.frame = 0;
                }
            }
            self.timer = 0.0;
        }
    }

    /// 해당 캐릭터를 canvas에 노출합니다.
    pub fn render(&self, canvas: &mut WindowCanvas, camera: &Rect, texture: &Texture) {
        let animation = self.animation.get(&self.direction).unwrap();
        let src: Rect = animation.0[self.frame as usize];

        // 캐릭터의 w, h는 VIRTUAL_WIDTH, VIRTUAL_HEIGHT 크기의 화면에 출력된다고 가정
        // 해당하는 w, h를 SCREEN_WIDTH, SCREEN_HEIGHT에 맞추어 출력해야한다.
        // w => w * SCREEN_WIDTH / VIRTUAL_WIDTH
        // h => h * SCREEN_HEIGHT / VIRTUAL_HEIGHT

        let transformed_rect = Rect::new(
            transform_value(self.x as i32 - camera.x, WIDTH_RATIO),
            transform_value(self.y as i32 - camera.y, HEIGHT_RATIO),
            transform_value(self.w, WIDTH_RATIO),
            transform_value(self.h, HEIGHT_RATIO),
        );
        canvas
            .copy_ex(
                texture,
                Some(src),
                Some(transformed_rect),
                0.,
                None,
                animation.1,
                animation.2,
            )
            .unwrap();

        // hitbox 그리기
        let hitbox_transformed_rect = Rect::new(
            transform_value(self.get_hitbox().unwrap().x() - camera.x, WIDTH_RATIO),
            transform_value(self.get_hitbox().unwrap().y - camera.y, HEIGHT_RATIO),
            transform_value(self.get_hitbox().unwrap().width(), WIDTH_RATIO),
            transform_value(self.get_hitbox().unwrap().height(), HEIGHT_RATIO),
        );

        canvas.set_draw_color(Color::RGBA(0, 255, 0, 255));
        canvas.draw_rect(hitbox_transformed_rect).unwrap();
    }
}
