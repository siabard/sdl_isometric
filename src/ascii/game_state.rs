//! 게임의 데이터를 보관하는 데이터셋

use super::{Component, Entity};
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    pub entities: Vec<Entity>,
    pub component: Component,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            entities: vec![],
            component: Component::default(),
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
}
