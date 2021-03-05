use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use std::collections::HashMap;

use crate::constant::*;
/// Tiled를 읽어서 맵을 채운다.
use crate::tile;
use sdl2::video::WindowContext;
use sdl2::{image::LoadTexture, render::Texture, render::TextureCreator};
use std::fs::File;
use std::path::Path;
use tiled::parse_file;

/// 맵의 가로 타일 수
pub const MAP_WIDTH: i32 = 20;

/// 맵의 세로 타일 수
pub const MAP_HEIGHT: i32 = 15;

/// 지도용 구조체
/// 지도에는 map용 파일과
/// 각 map 블럭에 대한 정보를 넣는다.
pub struct Map<'a> {
    map_id: String,
    pub x: i32,     //  x
    pub y: i32,     //  y
    pub cam_x: i32, // camera_x
    pub cam_y: i32, // camera_y
    pub tile_atlases: HashMap<usize, tile::TileAtlas>,
    pub width: u32,  // total number of tile in horizontal in this map
    pub height: u32, // total numbr of tile in vertical in this map
    pub tile_widths: HashMap<usize, u32>, // width of a tile in pixels
    pub tile_heights: HashMap<usize, u32>, // height of a tile in pixels
    pub layers: Vec<tiled::Layer>,
    pub textures: HashMap<usize, Texture<'a>>,
    pub blocks: HashMap<i32, Rect>,
}

impl<'a> Map<'a> {
    pub fn new(
        map_id: String,
        texture_creator: &'a TextureCreator<WindowContext>,
        path: &'static str,
    ) -> Map<'a> {
        // read tmx file
        let map: tiled::Map = parse_file(&Path::new(&(ASSET_DIR.to_owned() + path))).unwrap();

        let layers: Vec<tiled::Layer> = map.layers;
        let tile_sets: Vec<tiled::Tileset> = map.tilesets;

        let mut textures = HashMap::new();
        let mut tile_atlases = HashMap::new();
        let mut tile_widths = HashMap::new();
        let mut tile_heights = HashMap::new();

        for (i, tileset) in tile_sets.iter().enumerate() {
            let tile_width = tileset.tile_width;
            let tile_height = tileset.tile_height;

            let texture = texture_creator
                .load_texture(Path::new(
                    &(ASSET_DIR.to_owned() + &tileset.images[0].source),
                ))
                .unwrap();

            let tile_atlas =
                tile::TileAtlas::new(&texture, tileset.first_gid, tile_width, tile_height);
            textures.insert(i, texture);
            tile_atlases.insert(i, tile_atlas);
            tile_widths.insert(i, tileset.tile_width);
            tile_heights.insert(i, tileset.tile_height);
        }
        Map {
            map_id,
            x: 0,
            y: 0,
            cam_x: 0,
            cam_y: 0,
            tile_atlases,
            width: map.width,
            height: map.height,
            tile_widths,
            tile_heights,
            layers,
            textures,
            blocks: HashMap::new(),
        }
    }

    pub fn add_blocks(&mut self, block: i32, x: i32, y: i32, w: u32, h: u32) {
        self.blocks.insert(block, Rect::new(x, y, w, h));
    }

    /// translate position (left, top) to tile
    /// map is display rom x, y
    pub fn point_to_tile(&self, tile_index: usize, left: i32, top: i32) -> (i32, i32) {
        let o_x = self.x.max(left);
        let o_y = self.y.max(top);

        let tile_width = *self.tile_widths.get(&tile_index).unwrap();
        let tile_height = *self.tile_heights.get(&tile_index).unwrap();
        let clamp_x = o_x.min(left + (self.width * tile_width) as i32 - 1);
        let clamp_y = o_y.min(top + (self.height * tile_height) as i32 - 1);

        let tile_x = (clamp_x - self.x) / tile_width as i32;
        let tile_y = (clamp_y - self.y) / tile_height as i32;

        (tile_x, tile_y)
    }

    pub fn render(&self, canvas: &mut WindowCanvas, camera_rect: &Rect) {
        for (i, layer) in self.layers.iter().enumerate() {
            if let tiled::LayerData::Finite(tiles) = &layer.tiles {
                let (tile_left, tile_top) = self.point_to_tile(i, camera_rect.x, camera_rect.y);
                let (tile_right, tile_bottom) = self.point_to_tile(
                    i,
                    camera_rect.x + camera_rect.w,
                    camera_rect.y + camera_rect.h,
                );

                // 카메라의 좌측/위가 타일에 정확히 일치한다면 tile_start_x와 tile_start_y는 0이 되겠지만
                // 그렇지 않은 경우는 좌측/위 타일에서 떨어진 좌표값만큼을 반환하게된다.
                // 이 값은 대상 texture의 영역을 어디에 노출시킬까 정할 때, 대상의 타일을 tile_start_x, tile_start_y만큼
                // 좌상단으로 올림으로써 부드러운 스크롤을 가능하게한다.
                let tile_width = *self.tile_widths.get(&i).unwrap();
                let tile_height = *self.tile_heights.get(&i).unwrap();

                let tile_start_x = camera_rect.x - tile_left * tile_width as i32;
                let tile_start_y = camera_rect.y - tile_top * tile_height as i32;

                for y in tile_top..tile_bottom {
                    for x in tile_left..tile_right {
                        let gid = tiles[y as usize][x as usize].gid;
                        if gid != 0 {
                            let rect = self.tile_atlases.get(&i).unwrap().get_tile_rect(gid - 1);

                            canvas
                                .copy_ex(
                                    &self.textures[&i],
                                    Some(rect),
                                    Some(Rect::new(
                                        (x - tile_left) as i32 * tile_width as i32 - tile_start_x,
                                        (y - tile_top) as i32 * tile_height as i32 - tile_start_y,
                                        tile_width,
                                        tile_height,
                                    )),
                                    0.0,
                                    None,
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
}
