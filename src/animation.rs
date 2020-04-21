use crate::constant::*;
use crate::*;

use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::WindowCanvas;

/// Animation 을 수행하는 내역
/// 개별 캐릭터는 하나의 UnitCharacter 이다.
#[derive(Clone, PartialEq)]
pub struct UnitCharacter {
    hitbox: Option<Rect>,
    animation: Vec<Rect>,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    frame: u32,
    max_frame: u32,
    timer: f64,
    pub span: f64,   // 한 프레임에 필요한 시간
    pub fliph: bool, // 좌우반전
    pub flipv: bool, // 세로반전
}

impl UnitCharacter {
    /// 개별 캐릭터를 등록한다.
    pub fn new(w: u32, h: u32, max_frame: u32, fliph: bool, flipv: bool) -> UnitCharacter {
        UnitCharacter {
            hitbox: None,
            animation: vec![],
            x: 0,
            y: 0,
            w: w,
            h: h,
            frame: 0,
            max_frame: max_frame,
            timer: 0.0f64,
            span: 1.0 / 4.0, // 0.25초마당 한 프레임, 즉 초당 4 프레임을 움직인다.
            fliph: fliph,
            flipv: flipv,
        }
    }

    /// hitbox를 등록한다.
    pub fn set_hitbox(&mut self, x: i32, y: i32, w: u32, h: u32) {
        self.hitbox = Some(Rect::new(x, y, w, h));
    }

    /// animation을 등록한다.
    pub fn set_animation(&mut self, frames: Vec<Rect>) {
        self.animation = frames;
    }

    /// 해당 캐릭터를 움직이게한다.
    pub fn update(&mut self, dt: f64) {
        // timer에 dt를 누적해서 span보다 커지면 한 프레임씩 증가한다.
        // 이렇게 하면 1초에 몇프레임 식으로 애니메이션을 조작할 수 있다.

        self.timer += dt;

        if self.timer > self.span {
            self.frame = self.frame + 1;
            if self.frame >= self.max_frame {
                self.frame = 0;
            }
            self.timer = 0.0;
        }
    }

    /// 해당 캐릭터를 canvas에 노출합니다.
    pub fn render(&self, canvas: &mut WindowCanvas, texture: &Texture) {
        let src: Rect = self.animation[self.frame as usize];

        // 캐릭터의 w, h는 VIRTUAL_WIDTH, VIRTUAL_HEIGHT 크기의 화면에 출력된다고 가정
        // 해당하는 w, h를 SCREEN_WIDTH, SCREEN_HEIGHT에 맞추어 출력해야한다.
        // w => w * SCREEN_WIDTH / VIRTUAL_WIDTH
        // h => h * SCREEN_HEIGHT / VIRTUAL_HEIGHT

        let transformed_rect = Rect::new(
            transform_value(self.x, WIDTH_RATIO),
            transform_value(self.y, HEIGHT_RATIO),
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
                self.fliph,
                self.flipv,
            )
            .unwrap();
    }
}
