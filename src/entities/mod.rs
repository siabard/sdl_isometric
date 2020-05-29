pub mod entity;
pub use components::*;
pub use entity::*;

use crate::*;

/// Facing 값에 따라 캐릭터 애니메이션을 설정한다.
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

/// e1, e2 컴포넌트간의 차이를 이용하여 e1이 e2를 바라보는 facing 벡터를 만든다.
pub fn facing_from_to(e1: &MovementComponent, e2: &MovementComponent) -> Vector2<f64> {
    let pos1 = e1.get_pos();
    let pos2 = e2.get_pos();

    let dx = pos1.0 - pos2.0;
    let dy = pos2.1 - pos1.1;
    let distance = (dx.powf(2.0) + dy.powf(2.0)).sqrt();
    let ratio_x = dx / distance;
    let ratio_y = dy / distance;
    let deg_x = ratio_x.acos() * 180. / std::f64::consts::PI;
    let deg_y = ratio_y.asin() * 180. / std::f64::consts::PI;

    let vector_x = (deg_x * std::f64::consts::PI / 180.0).cos();
    let vector_y = -(deg_y * std::f64::consts::PI / 180.0).sin();

    (vector_x, vector_y)
}

/// ENTITY상수값
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EntityType {
    PLAYER,
    MOB,
    ATTACK,
    BLOCK,
}
