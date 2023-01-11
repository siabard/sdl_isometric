//! 게임의 데이터를 보관하는 데이터셋

use super::{Component, Entity, Grid};
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    pub entities: Vec<Entity>,
    pub component: Component,
    pub grids: Vec<Grid>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            entities: vec![],
            component: Component::default(),
            grids: vec![],
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

        for _ in 0..NUM_TRIES {
            let grid = Grid::new(
                rng.gen_range(0, 320 / 8),
                rng.gen_range(0, 240 / 16),
                rng.gen_range(3, 8),
                rng.gen_range(3, 8),
            );

            if self.grids.len() == 0 || self.grids.iter().find(|&g| g.aabb(&grid)).is_none() {
                self.grids.push(grid);
            }
        }
    }
}
