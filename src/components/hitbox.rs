use crate::constant::*;
use crate::*;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

/// hitbox coordinates
#[derive(Clone, Copy, Debug)]
pub struct HitboxComponent {
    pub x: f64,
    pub y: f64,
    pub hx: f64,
    pub hy: f64,
    pub w: f64,
    pub h: f64,
}

impl HitboxComponent {
    pub fn new(x: f64, y: f64, hx: f64, hy: f64, w: f64, h: f64) -> HitboxComponent {
        HitboxComponent { x, y, hx, hy, w, h }
    }

    pub fn update(&mut self, _dt: f64, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn render(&self, canvas: &mut WindowCanvas, camera: &Rect) {
        // draw hitbox
        let hitbox_transformed_rect = Rect::new(
            transform_value((self.x + self.hx) as i32 - camera.x, WIDTH_RATIO),
            transform_value((self.y + self.hy) as i32 - camera.y, HEIGHT_RATIO),
            transform_value(self.w as u32, WIDTH_RATIO),
            transform_value(self.h as u32, HEIGHT_RATIO),
        );

        canvas.set_draw_color(Color::RGBA(255, 0, 0, 200));
        canvas.draw_rect(hitbox_transformed_rect).unwrap();
    }

    pub fn get_rect(&self) -> Rectangle {
        Rectangle {
            x: self.x + self.hx,
            y: self.y + self.hy,
            w: self.w,
            h: self.h,
        }
    }

    pub fn is_collide(&mut self, hitbox: &HitboxComponent) -> bool {
        detect_collision(&self.get_rect(), &hitbox.get_rect())
    }
}
