use crate::constant::*;
use crate::*;
use sdl2::image::LoadTexture;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::video::WindowContext;
use std::collections::HashMap;
use std::path::Path;

/// Texture를 저장하는 객체
pub struct TextureManager<'a> {
    pub textures: HashMap<String, Texture<'a>>,
}

impl<'a> TextureManager<'a> {
    pub fn new() -> TextureManager<'a> {
        TextureManager {
            textures: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.textures.clear();
    }

    /// 이미지로부터 텍스쳐를 생성하여 가져옴
    pub fn load_texture(
        &mut self,
        texture_id: String,
        texture_creator: &'a TextureCreator<WindowContext>,
        path: &Path,
    ) {
        let texture = texture_creator.load_texture(path).unwrap();
        self.textures.insert(texture_id, texture);
    }

    /// Texture 를 추가함
    pub fn add_texture(&mut self, texture_id: String, texture: Texture<'a>) {
        self.textures.insert(texture_id, texture);
    }

    /// Texture 를 삭제함
    pub fn remove_texture(&mut self, texture_id: String) {
        self.textures.remove(&texture_id);
    }
}

/// Texture의 일부를 Sprite로 처리하는 부분
#[derive(Clone)]
pub struct Sprite {
    texture_id: String,
    sprite_id: String,
    pub src: Rect,
    pub dest: Rect,
    rotation: f64,
    orientation: Option<Point>,
    flip_h: bool,
    flip_v: bool,
}

impl Sprite {
    pub fn new(
        texture_id: String,
        sprite_id: String,
        src: Rect,
        dest: Rect,
        orientation: Option<Point>,
        rotation: f64,
        flip_h: bool,
        flip_v: bool,
    ) -> Sprite {
        Sprite {
            texture_id,
            sprite_id,
            src,
            dest,
            rotation,
            orientation,
            flip_h,
            flip_v,
        }
    }

    /// 스프라이트에 대한 랜더링
    /// 가상화면에 출력하는 것을 전제로 함
    pub fn render(&self, canvas: &mut WindowCanvas, texture_manager: &TextureManager) {
        let texture = texture_manager.textures.get(&self.texture_id).unwrap();

        canvas
            .copy_ex(
                texture,
                Some(self.src),
                Some(transform_rect(&self.dest, WIDTH_RATIO, HEIGHT_RATIO)),
                self.rotation,
                self.orientation,
                self.flip_h,
                self.flip_v,
            )
            .unwrap();
    }
}
