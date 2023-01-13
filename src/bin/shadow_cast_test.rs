use sdl_isometric::physics::shadow_casting::*;

fn main() {
    let row: Row = Row::new(2, 1., -1.);
    let row_south: Row = Row::new(2, -1., 1.);
    let row_east: Row = Row::new(2, 1., -1.);
    let row_west: Row = Row::new(2, -1., 1.);

    let mut light_map: LightMap = LightMap::new(10, 4);
    light_map.scan(Direction::North, (2, 2), &row);
    light_map.scan(Direction::South, (2, 2), &row_south);
    light_map.scan(Direction::East, (2, 2), &row_east);
    light_map.scan(Direction::West, (2, 2), &row_west);
}
