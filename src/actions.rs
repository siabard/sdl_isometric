use crate::entities::*;
use crate::map::*;

pub fn teleport(map: &Map, entity: &mut Entity, tile_x: u32, tile_y: u32) {
    let (new_x, new_y) = map.get_tile_xy(tile_x, tile_y);
    entity.set_pos_xy(new_x, new_y);
}
