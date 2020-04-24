use num_traits::cast::{FromPrimitive, ToPrimitive};
use num_traits::int::PrimInt;

use sdl2::rect::Rect;
use std::collections::HashSet;

/// 방향에 대한 enum
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}

/// 캐릭터 분류
pub mod character {
    pub const PLAYER: &'static str = "player";
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
pub fn detect_collision(p1: &Rect, p2: &Rect) -> HashSet<Direction> {
    let mut directions = HashSet::new();
    if p1.x < p2.x + p2.width() as i32
        && p1.x + p1.width() as i32 > p2.x
        && p1.y < p2.y + p2.height() as i32
        && p1.y + p1.height() as i32 > p2.y
    {
        // p1 을 기준으로 p1 의 어느 쪽이 p2와 충돌했는지 검사
        if p1.x < p2.x + p2.width() as i32 {
            directions.insert(Direction::Left);
        }
        if p1.x + p1.width() as i32 > p2.x {
            directions.insert(Direction::Right);
        }
        if p1.y < p2.y + p2.height() as i32 {
            directions.insert(Direction::Up);
        }

        if p1.y + p1.height() as i32 > p2.y {
            directions.insert(Direction::Down);
        }
    }

    directions
}

pub mod animation;
pub mod constant;
pub mod gui;
pub mod map;
pub mod states;
pub mod texture_manager;
