use num_traits::cast::{FromPrimitive, ToPrimitive};
use num_traits::int::PrimInt;

use sdl2::rect::Rect;

/// 방향에 대한 enum
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}

/// 캐릭터 분류
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Character {
    Player = 0,
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

pub mod animation;
pub mod constant;
pub mod gui;
pub mod states;
