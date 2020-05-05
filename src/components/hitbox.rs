use crate::constant::*;
use crate::*;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

/// 충돌 좌표를 가지고 있는 부분
#[derive(Clone, Copy, Debug)]
pub struct HitboxComponent {
    pub x: f64,
    pub y: f64,
    pub hx: f64,
    pub hy: f64,
    pub w: u32,
    pub h: u32,
}

impl HitboxComponent {
    pub fn new(x: f64, y: f64, hx: f64, hy: f64, w: u32, h: u32) -> HitboxComponent {
        HitboxComponent { x, y, hx, hy, w, h }
    }

    pub fn update(&mut self, _dt: f64, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn render(&self, canvas: &mut WindowCanvas, camera: &Rect) {
        // hitbox 그리기
        let hitbox_transformed_rect = Rect::new(
            transform_value((self.x + self.hx) as i32 - camera.x, WIDTH_RATIO),
            transform_value((self.y + self.hy) as i32 - camera.y, HEIGHT_RATIO),
            transform_value(self.w, WIDTH_RATIO),
            transform_value(self.h, HEIGHT_RATIO),
        );

        canvas.set_draw_color(Color::RGBA(0, 255, 0, 255));
        canvas.draw_rect(hitbox_transformed_rect).unwrap();
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(
            (self.x + self.hx) as i32,
            (self.y + self.hy) as i32,
            self.w,
            self.h,
        )
    }

    pub fn is_collide(&mut self, hitbox: &HitboxComponent) -> bool {
        detect_collision(&self.get_rect(), &hitbox.get_rect())
    }
}
