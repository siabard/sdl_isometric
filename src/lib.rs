#[macro_use]
extern crate lazy_static;

pub mod ai;
pub mod animation;
pub mod components;
pub mod constant;
pub mod entities;
pub mod gui;
pub mod map;
pub mod physics;
pub mod quadtree;
pub mod states;
pub mod texture_manager;
pub mod timer;
pub mod tween;

pub use states::game_state::*;
pub use states::init_state::*;
pub use states::timer_state::*;

use num_traits::cast::{FromPrimitive, ToPrimitive};
use num_traits::int::PrimInt;

use sdl2::rect::Rect;

///2차원 배열
type Vector2<V> = (V, V);

/// Data Structure : Rectangle
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Rectangle {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

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
    pub const ENEMY: &str = "enemy";
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

/// collision detection (AABB)
pub fn detect_collision(p1: &Rectangle, p2: &Rectangle) -> bool {
    p1.x < p2.x + p2.w && p1.x + p1.w > p2.x && p1.y < p2.y + p2.h && p1.y + p1.h > p2.y
}

/// 충돌이 일어날 때 이동벡터를 계산한다.
pub fn calc_vector(m1: &Rectangle, v1: Vector2<f64>, m2: &Rectangle) -> Vector2<f64> {
    // m1과 m2의 x, y접점의 길이를 구한다.
    let dx = if m1.x < m2.x {
        m1.x + m1.w - m2.x
    } else {
        m2.x + m2.w - m1.x
    };

    let dy = if m1.y < m2.y {
        m1.y + m1.h - m2.y
    } else {
        m2.y + m2.h - m1.y
    };

    // 접한면이 큰 방향으로 Slide한다.
    // 만약 dx가 크다면 slide는 좌우로 진행된다.
    let anti_vector = if dx > dy {
        // 위 아니면, 아래
        if m1.y > m2.y {
            // m2는 m1보다 위에 있으므로 아랫면에 충돌한 것이다.
            (0.0, 1.0)
        } else {
            // 위
            (0.0, -1.0)
        }
    } else if dx < dy {
        // 왼쪽 아래면, 오른쪽
        if m1.x > m2.x {
            (1.0, 0.0)
        } else {
            (-1.0, 0.0)
        }
    } else {
        (0.0, 0.0)
    };

    // dp = dp - N *dot(dp, N)
    let dot = v1.0 * anti_vector.0 + v1.1 * anti_vector.1;

    let result = (v1.0 - anti_vector.0 * dot, v1.1 - anti_vector.1 * dot);

    //dbg!(v1, anti_vector, result);
    result
}
