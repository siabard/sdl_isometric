//! 게임의 데이터를 보관하는 데이터셋

use crate::physics::shadow_casting::LightMap;

use super::{Component, Entity, Grid, Tile};
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    pub entities: Vec<Entity>,
    pub component: Component,
    pub visibility: Vec<bool>,
}

impl GameState {
    pub fn new(width: i32, height: i32) -> Self {
        let mut visibility = vec![];
        visibility.resize(width as usize * height as usize, false);

        GameState {
            entities: vec![],
            component: Component::default(),
            visibility,
        }
    }

    pub fn add_entity(&mut self, coord: Option<super::Coord>, tile: Option<super::Tile>) -> Entity {
        // entities 에는 없는 u32 값을 만든다.
        let mut rng = thread_rng();
        let mut x: u32 = rng.gen();

        while self.entities.iter().find(|e| **e == x).is_some() {
            x = rng.gen();
        }

        self.entities.push(x);

        if let Some(c) = coord {
            self.component.coord.insert(x, c);
        }

        if let Some(t) = tile {
            self.component.tile.insert(x, t);
        }
        x
    }

    pub fn remove_entity(&mut self, entity: &Entity) {
        self.component.coord.remove(entity);
        self.component.tile.remove(entity);
        self.entities.retain(|e| e != entity);
    }

    pub fn entity_coord_and_tile(&self) -> Vec<(&super::Coord, &super::Tile)> {
        self.entities
            .iter()
            .filter(|&e| {
                self.component.coord.get(e).is_some() && self.component.tile.get(e).is_some()
            })
            .map(|e| {
                (
                    self.component.coord.get(e).unwrap(),
                    self.component.tile.get(e).unwrap(),
                )
            })
            .collect::<Vec<(&super::Coord, &super::Tile)>>()
    }

    pub fn entity_coord_update(&mut self, entity: u32, coord: Option<super::Coord>) {
        if let Some(c) = coord {
            if let Some(entity_coord) = self.component.coord.get_mut(&entity) {
                *entity_coord = c;
            }
        }
    }

    pub fn generate_rooms(&mut self) {
        const NUM_TRIES: u32 = 100;
        let mut rng = thread_rng();
        let mut grids: Vec<Grid> = vec![];

        for _ in 0..NUM_TRIES {
            let grid = Grid::new(
                rng.gen_range(1, 320 / 8 - 1),
                rng.gen_range(1, 240 / 16 - 1),
                rng.gen_range(5, 9),
                rng.gen_range(5, 9),
            );

            if (grid.x + grid.w as i32) < (320 / 8) && (grid.y + grid.h as i32) < (240 / 16) {
                if grids.len() == 0 || grids.iter().find(|&g| g.aabb(&grid)).is_none() {
                    grids.push(grid);
                }
            }
        }

        // grids 내용을 토대로 entities 에 내역을 넣는다.
        // 가로 320 / 8(40), 세로 240 / 16 (15) 의 셀이 된다.
        // 해당 셀을 floor 로 등록하거나 Wall 로 등록해야한다.

        for y in 0..(240 / 16) {
            for x in 0..(320 / 8) {
                // 해당하는 셀이 grids 에 포함되면 Floor
                // grids 에 포함되지않으면 Wall 이다.

                let grid = Grid::new(x, y, 1u32, 1u32);
                if grids.iter().find(|&g| g.aabb(&grid)).is_some() {
                    self.add_entity(Some((x, y)), Some(Tile::Floor));
                } else {
                    self.add_entity(Some((x, y)), Some(Tile::Wall));
                }
            }
        }

        self.make_corridor(&grids);
    }

    /// 임의의 좌표에 어떤 entity가 있는지 반환한다.
    pub fn get_entity_on_coord(&self, x: i32, y: i32) -> Vec<Entity> {
        self.component
            .coord
            .iter()
            .filter(|&c| c.1 .0 == x && c.1 .1 == y)
            .map(|c| *(c.0))
            .collect()
    }

    /// 복도를 만든다.
    pub fn make_corridor(&mut self, grids: &Vec<Grid>) {
        // 임의의 Grid 2개를 뽑아서...
        // 해당 Grid 의 중점을 연결하는 선을 긋는다.
        let mut rng = thread_rng();
        const NUM_TRIES: u32 = 100;

        for _ in 0..NUM_TRIES {
            let i = rng.gen_range(0, grids.len());
            let j = rng.gen_range(0, grids.len());

            if i != j {
                let center_1 = (
                    grids[i].x + grids[i].w as i32 / 2,
                    grids[i].y + grids[i].h as i32 / 2,
                );
                let center_2 = (
                    grids[j].x + grids[j].w as i32 / 2,
                    grids[j].y + grids[j].h as i32 / 2,
                );

                // center_1 에서 시작해서 center_2까지 일단 가로로 지운다.
                // 왼쪽에 있는 상자가 기준이다.

                let (start_x, end_x) = if center_1.0 > center_2.0 {
                    (center_2.0, center_1.0)
                } else {
                    (center_1.0, center_2.0)
                };
                let (start_y, end_y) = if center_1.0 > center_2.0 {
                    (center_2.1, center_1.1)
                } else {
                    (center_1.1, center_2.1)
                };

                for x in start_x..end_x {
                    // 해당 좌표에 셀이 있는지 검사
                    let entities = self.get_entity_on_coord(x, start_y);

                    // 해당 좌표에 있는 모든 셀 을지운다.
                    if entities.len() != 0 {
                        for e in entities.iter() {
                            self.remove_entity(e);
                        }
                    }

                    self.add_entity(Some((x, start_y)), Some(Tile::Floor));
                }

                let iter_y = if start_y < end_y {
                    start_y..end_y
                } else {
                    end_y..start_y
                };

                for y in iter_y {
                    // 해당 좌표에 셀이 있는지 검사
                    let entities = self.get_entity_on_coord(end_x, y);

                    // 해당 좌표에 있는 모든 셀을 지운다.
                    if entities.len() != 0 {
                        for e in entities.iter() {
                            self.remove_entity(e);
                        }
                    }

                    self.add_entity(Some((end_x, y)), Some(Tile::Floor));
                }
            }
        }
    }

    /// 맵의 일부 지역을 밝힌다.
    pub fn update_visiblity(&mut self, lightmap: &LightMap) {
        for (i, v) in lightmap.visible.iter().enumerate() {
            let vis = self.visibility.get_mut(i).unwrap();

            if *v {
                *vis = *v;
            }
        }
    }
}
