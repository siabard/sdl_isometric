use crate::constant::*;
use crate::texture_manager::*;
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

const MAP_WIDTH: i32 = 20;
const MAP_HEIGHT: i32 = 15;

/// 지도용 구조체
/// 지도에는 map용 파일과
/// 각 map 블럭에 대한 정보를 넣는다.
pub struct Map {
    map_id: String,
    blocks: HashMap<i32, Rect>,
    map: Vec<i32>,
    cell_width: u32,
    cell_height: u32,
}

impl Map {
    pub fn new(map_id: String, w: u32, h: u32) -> Map {
        Map {
            map_id,
            blocks: HashMap::new(),
            map: vec![],
            cell_width: w,
            cell_height: h,
        }
    }

    pub fn init_map(&mut self, block: i32, x: i32, y: i32, w: u32, h: u32) {
        self.blocks.insert(block, Rect::new(x, y, w, h));
    }

    pub fn load_map(&mut self) {
        // 0 -> water
        // 1 -> grass
        // 2 -> sand
        self.map = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 1, 2, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 2, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 1, 2, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ];
    }

    pub fn render(&self, canvas: &mut WindowCanvas, texture_manager: &TextureManager) {
        let texture_ref = texture_manager.textures.get(&self.map_id).unwrap();
        let texture = texture_ref.borrow();
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let idx = (y * MAP_WIDTH + x) as usize;
                if let Some(map_value) = self.map.get(idx) {
                    if let Some(map) = self.blocks.get(map_value) {
                        canvas
                            .copy_ex(
                                &texture,
                                Some(*map),
                                Some(transform_rect(
                                    &Rect::new(
                                        x * self.cell_width as i32,
                                        y * self.cell_height as i32,
                                        self.cell_width,
                                        self.cell_height,
                                    ),
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
                }
            }
        }
    }
}
