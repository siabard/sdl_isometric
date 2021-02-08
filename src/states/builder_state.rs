use crate::components::*;
use crate::constant::*;
use crate::entities::*;
use crate::map::*;
use crate::states::*;

use std::collections::HashMap;
use std::collections::HashSet;

use sdl2::mixer::Chunk;
use sdl2::mixer::Music;

use uuid::Uuid;

/// 빌드 게임용 State
pub struct BuilderState<'a> {
    texture_manager: TextureManager<'a>,
    entities: HashMap<Uuid, Entity>,
    music: Option<Music<'a>>,
    chunks: HashMap<String, Chunk>,
    map: Option<Map<'a>>,
    state_result: StateResult,
    keyboards: HashSet<sdl2::keyboard::Keycode>,
    cx: i32, // 카메라 x좌표
    cy: i32, // 카메라 Y좌표
    cw: u32, // 카메라 폭
    ch: u32, // 카메라 높이
}

impl<'a> BuilderState<'a> {
    pub fn new() -> BuilderState<'a> {
        let texture_manager = TextureManager::new();
        let mut entities = HashMap::new();

        BuilderState {
            texture_manager,
            entities,
            state_result: StateResult::Default,
            map: None,
            music: None,
            chunks: HashMap::new(),
            keyboards: HashSet::new(),
            cx: 0,
            cy: 0,
            cw: VIRTUAL_WIDTH,
            ch: VIRTUAL_HEIGHT,
        }
    }
}

impl<'a> States for BuilderState<'a> {
    fn process_event(&mut self, event: &sdl2::event::Event, dt: f64) -> StateResult {
        StateResult::Default
    }

    fn process_mouse(
        &mut self,
        x: i32,
        y: i32,
        new_buttons: &HashSet<sdl2::mouse::MouseButton>,
        old_buttons: &HashSet<sdl2::mouse::MouseButton>,
        dt: f64,
    ) {
    }

    fn update(&mut self, dt: f64) -> StateResult {
        StateResult::Default
    }

    fn render(&self, canvas: &mut WindowCanvas) -> StateResult {
        StateResult::Default
    }

    fn next_result(&mut self) -> StateResult {
        StateResult::Default
    }
}
