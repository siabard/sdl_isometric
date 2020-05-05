use crate::animation::*;
use crate::texture_manager::*;
use crate::*;

use std::collections::HashSet;

use sdl2::pixels::Color;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;

pub trait States {
    ///  키 입력 등 일반적인 부분의 처리
    fn process_event(&mut self, event: &sdl2::event::Event, dt: f64) -> StateResult;

    /// 마우스 입력부분만 여기서 처리
    fn process_mouse(
        &mut self,
        x: i32,
        y: i32,
        new_buttons: &HashSet<sdl2::mouse::MouseButton>,
        old_buttons: &HashSet<sdl2::mouse::MouseButton>,
        dt: f64,
    );

    /// state 값을 변경시키는 부분에 대한 처리
    fn update(&mut self, dt: f64) -> StateResult;

    /// 화면에 노출시키기
    fn render(&self, canvas: &mut WindowCanvas) -> StateResult;

    /// main loop에서 States의 다음 상태를 요청할 때
    fn next_result(&mut self) -> StateResult;
}

pub mod game_state;
pub mod init_state;
