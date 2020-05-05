pub mod entity;
pub use entity::*;

use crate::*;

pub fn facing_to_direction(facing: Vector2<i32>) -> Direction {
    if facing.0 > 0 {
        Direction::Right
    } else if facing.0 < 0 {
        Direction::Left
    } else if facing.1 < 0 {
        Direction::Up
    } else if facing.1 > 0 {
        Direction::Down
    } else {
        Direction::Stop
    }
}

/// ENTITY상수값
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EntityType {
    PLAYER,
    MOB,
    ATTACK,
    BLOCK,
}
