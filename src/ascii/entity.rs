//! Entity 데이터들

use hangul_jaso::Languages;
use jaso_sdl2::Fonts;

/// Entity 기본형
pub type Entity = u32;

/// 좌표의 기본형
pub type Coord = (i32, i32);

/// 타일의 기본형
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Tile {
    Wall,
    Floor,
    Player,
    Ascii(char),
    Hangul(char),
    Arrow(char),
    Grid(char),
    Blank,
}

/// 콤포넌트 타입
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Component {
    pub coord: std::collections::HashMap<u32, Coord>,
    pub tile: std::collections::HashMap<u32, Tile>,
}

/// 데이터를 보관하는 구조체
pub struct AppData {
    pub entities: Vec<Entity>,
    pub component: Component,
}

/// View를 담당할 구조체
pub struct AppView {
    pub fonts: std::collections::HashMap<Languages, Fonts>,
}
