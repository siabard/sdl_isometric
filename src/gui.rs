use crate::constant::*;
use crate::*;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::video::WindowContext;
use std::collections::HashSet;
use std::path::Path;
use uuid::Uuid;

/// 화면 입력을 컨트롤할 수 있는
/// GUI객체
pub struct GuiElement<'a> {
    id: Uuid,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    texture_normal: Texture<'a>,
    texture_hover: Texture<'a>,
    pub is_hover: bool,
    pub is_clicked: bool,
}

impl<'a> GuiElement<'a> {
    pub fn new(
        id: Uuid,
        texture_creator: &'a TextureCreator<WindowContext>,
        normal_path: &Path,
        hover_path: &Path,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
    ) -> GuiElement<'a> {
        let texture_normal = texture_creator.load_texture(normal_path).unwrap();
        let texture_hover = texture_creator.load_texture(hover_path).unwrap();

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

    pub fn render(&self, canvas: &mut WindowCanvas) {
        canvas
            .copy_ex(
                if self.is_hover {
                    &self.texture_hover
                } else {
                    &self.texture_normal
                },
                None,
                Some(transform_rect(
                    &Rect::new(self.x, self.y, self.w, self.h),
                    WIDTH_RATIO,
                    HEIGHT_RATIO,
                )),
                0.,
                Some(Point::new(0, 0)),
                false,
                false,
            )
            .unwrap();
    }

    pub fn reset(&mut self) {
        self.is_hover = false;
        self.is_clicked = false;
    }
}
