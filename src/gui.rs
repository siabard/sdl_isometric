use crate::constant::*;
use crate::texture_manager::*;
use crate::*;

use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::collections::HashSet;
use uuid::Uuid;

/// collision detection
pub fn detect_collision(p1: &Rect, p2: &Rect) -> bool {
    p1.x < p2.x + p2.width() as i32
        && p1.x + p1.width() as i32 > p2.x
        && p1.y < p2.y + p2.height() as i32
        && p1.y + p1.height() as i32 > p2.y
}

/// 화면 입력을 컨트롤할 수 있는
/// GUI객체
pub struct GuiElement {
    id: Uuid,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    texture_normal: Sprite,
    texture_hover: Sprite,
    pub is_hover: bool,
    pub is_clicked: bool,
}

impl GuiElement {
    pub fn new(
        id: Uuid,
        texture_normal: (String, String),
        texture_hover: (String, String),
        x: i32,
        y: i32,
        w: u32,
        h: u32,
    ) -> GuiElement {
        let texture_normal = Sprite::new(
            texture_normal.0,
            texture_normal.1,
            Rect::new(0, 0, w, h),
            Rect::new(x, y, w, h),
            None,
            0.0,
            false,
            false,
        );
        let texture_hover = Sprite::new(
            texture_hover.0,
            texture_hover.1,
            Rect::new(0, 0, w, h),
            Rect::new(x, y, w, h),
            None,
            0.0,
            false,
            false,
        );

        GuiElement {
            id,
            x,
            y,
            w,
            h,
            texture_normal,
            texture_hover,
            is_hover: false,
            is_clicked: false,
        }
    }

    pub fn update(&mut self) {}

    pub fn process_event(&mut self, _event: &Event) {}

    /// 마우스 입력부분만 여기서 처리
    pub fn process_mouse(
        &mut self,
        x: i32,
        y: i32,
        new_buttons: &HashSet<sdl2::mouse::MouseButton>,
        old_buttons: &HashSet<sdl2::mouse::MouseButton>,
    ) {
        // x와 y는 physical 데이터
        // self.x와 self.y 는 transform 된 데이터
        let rx = transform_value(x, REVERSE_WIDTH_RATIO);
        let ry = transform_value(y, REVERSE_HEIGHT_RATIO);

        self.is_hover = rx >= self.x
            && rx <= self.x + self.w as i32
            && ry >= self.y
            && ry <= self.y + self.h as i32;

        // 버튼 press 체크
        if self.is_hover && new_buttons.contains(&sdl2::mouse::MouseButton::Left) {
            //println!("Left Button is down");
        }

        // 버튼 release 체크
        if self.is_hover && old_buttons.contains(&sdl2::mouse::MouseButton::Left) {
            self.is_clicked = true;
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas, texture_manager: &TextureManager) {
        if self.is_hover {
            self.texture_hover.render(canvas, texture_manager);
        } else {
            self.texture_normal.render(canvas, texture_manager);
        }
    }

    pub fn reset(&mut self) {
        self.is_hover = false;
        self.is_clicked = false;
    }
}
