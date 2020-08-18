use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;

use std::convert::TryInto;

use crate::components::AnimationComponent;
use crate::constant::*;
use crate::timer::TimerResult;
use crate::*;

/// 공격 관련 처리
#[derive(Clone, Debug)]
pub struct AttackComponent {
    pub deg: i32,        //바라보는 각도
    attack_timer: f64,   // 공격 타이머
    pub attacking: bool, // 공격 시도 진행
}

impl AttackComponent {
    pub fn new() -> AttackComponent {
        AttackComponent {
            deg: 0,
            attack_timer: 0.0,
            attacking: false,
        }
    }

    pub fn attack(&mut self) {
        if !self.attacking {
            self.attacking = true;
            self.attack_timer = 0.;
        }
    }

    /// x,y좌표에 맞게 바라보는 각도를 맞춘다.
    pub fn set_deg(&mut self, (x, y): (f64, f64), comp: &AnimationComponent) {
        let d_x = x - (comp.x + (comp.w / 2) as f64);
        let d_y = (comp.y + (comp.h / 2) as f64) - y;

        let distance = (d_x * d_x + d_y * d_y).sqrt();
        let ratio = d_x / distance;
        let mut deg = ratio.acos() * 180. / std::f64::consts::PI;

        if d_y > 0. {
            deg = -deg;
        }

        self.deg = deg as i32;
    }

    pub fn update(&mut self, dt: f64) {
        if self.attacking {
            self.attack_timer += dt;
            if self.attack_timer > 2. {
                self.attacking = false;
                self.attack_timer = 0.;
            }
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas, camera: &Rect, comp: &AnimationComponent) {
        // 공격 가능한 영역 그리기
        // 공격 가능한 영역 그리기
        let center_x: f64 = (comp.x as i32 - camera.x + (comp.w as i32 / 2)) as f64;
        let center_y: f64 = (comp.y as i32 - camera.y + (comp.h as i32 / 2)) as f64;

        canvas
            .filled_pie(
                transform_value(center_x as i16, WIDTH_RATIO),
                transform_value(center_y as i16, HEIGHT_RATIO),
                32,
                (self.deg - 30).try_into().unwrap(),
                (self.deg + 30).try_into().unwrap(),
                if self.attacking {
                    Color::RGBA(
                        255,
                        255,
                        255,
                        (((2.0 - self.attack_timer) / 2.0) * 255.) as u8,
                    )
                } else {
                    Color::RGBA(255, 255, 255, 50)
                },
            )
            .unwrap();
    }
}
