//! Entity 데이터들

/// Entity 기본형
pub type Entity = Vector<u32>;

/// 좌표의 기본형
pub type Coord = (i32, i32);

/// 타일의 기본형
pub enum Tile {
    Wall,
    Floor,
    Player,
    Ascii(char),
    Hangul(char),
    Arrow(char),
    Grid(char),
}

pub struct Component {
    pub coord: std::collections::HashMap<u32, Coord>,
    pub tile: std::collections::HashMap<u32, Tile>,
}

pub struct AppData {
    pub entity: Entity,
    pub component: Component,
}
