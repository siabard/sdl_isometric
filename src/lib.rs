use num_traits::cast::{FromPrimitive, ToPrimitive};
use num_traits::int::PrimInt;

use sdl2::rect::Rect;

///
type Vector2<V> = (V, V);

/// 방향에 대한 enum
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    IdleLeft,
    IdleRight,
    IdleUp,
    IdleDown,
    Stop,
}

/// 캐릭터 분류
pub mod character {
    pub const PLAYER: &str = "player";
}

#[derive(PartialEq, Copy, Clone)]
pub enum StateInfo {
    Init(&'static str),
    Game(&'static str),
}

#[derive(PartialEq, Copy, Clone)]
pub enum StateResult {
    Push(StateInfo),
    Trans(StateInfo),
    Pop,
    Default,
}

pub fn transform_value<T>(src: T, ratio: f32) -> T
where
    T: PrimInt + ToPrimitive + FromPrimitive,
{
    let f_value = src.to_f32().unwrap();
    FromPrimitive::from_f32(f_value * ratio).unwrap()
}

pub fn transform_rect(src: &Rect, ratio_w: f32, ratio_h: f32) -> Rect {
    Rect::new(
        transform_value(src.x(), ratio_w),
        transform_value(src.y(), ratio_h),
        transform_value(src.width(), ratio_w),
        transform_value(src.height(), ratio_h),
    )
}

/// collision detection
pub fn detect_collision(p1: &Rect, p2: &Rect) -> bool {
    p1.x < p2.x + p2.width() as i32
        && p1.x + p1.width() as i32 > p2.x
        && p1.y < p2.y + p2.height() as i32
        && p1.y + p1.height() as i32 > p2.y
}

pub mod animation;
pub mod components;
pub mod constant;
pub mod entities;
pub mod gui;
pub mod map;
pub mod states;
pub mod texture_manager;
pub use states::game_state::*;
pub use states::init_state::*;
