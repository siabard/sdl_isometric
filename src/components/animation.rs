use crate::constant::*;
use crate::texture_manager::*;
use crate::*;
use sdl2::render::WindowCanvas;

/// 애니메이션을 위한 부분
#[derive(Clone, Debug)]
pub struct AnimationComponent {
    pub x: f64,
    pub y: f64,
    pub w: u32,
    pub h: u32,
    textures: Vec<String>,
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
        textures: Vec<String>,
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
            textures,
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

    pub fn render(
        &self,
        canvas: &mut WindowCanvas,
        camera: &Rect,
        texture_manager: &TextureManager,
    ) {
        let rect = Rect::new(
            transform_value(self.x as i32 - camera.x, WIDTH_RATIO),
            transform_value(self.y as i32 - camera.y, HEIGHT_RATIO),
            transform_value(self.w, WIDTH_RATIO),
            transform_value(self.h, WIDTH_RATIO),
        );
        let src = self.frames[self.frame as usize];

        for texture_key in &self.textures {
            let texture = texture_manager.textures.get(texture_key).unwrap();
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
}
