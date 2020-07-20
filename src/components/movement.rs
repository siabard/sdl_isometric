use crate::constant::*;
use crate::*;

/// 이동진행을 위한 부분
#[derive(Copy, Clone, Debug)]
pub struct MovementComponent {
    pub x: f64,  // 기준 x위치
    pub y: f64,  // 기준 y위치
    pub px: f64, // 다음 tick으로 이동할 때 x위치
    pub py: f64, // 다음 tick으로 이동할 때 y위치
    pub facing: Vector2<i32>,
    pub velocity: Vector2<f64>,
    max_velocity: f64,
    accelaration: f64,
    decelaration: f64,
}

impl MovementComponent {
    pub fn new(
        x: f64,
        y: f64,
        facing: Vector2<i32>,
        velocity: Vector2<f64>,
        max_velocity: f64,
        accelaration: f64,
        decelaration: f64,
    ) -> MovementComponent {
        MovementComponent {
            x,
            y,
            px: x,
            py: y,
            facing,
            velocity,
            max_velocity,
            accelaration,
            decelaration,
        }
    }

    pub fn update_velocity(&mut self, dt: f64) {
        // dt는 1000 밀리초(1초) 기준으로 한 프레임의 크기이다.
        // 해당 캐릭터는 dt 만큼 감속된 값으로 이동하게된다.
        // 이동 속도 벡터와 현재 속도를 비교하여 최대 이동폭을 구한다.

        if self.velocity.0 > 0. {
            // X 속도의 최대치 계산
            if self.velocity.0 > self.max_velocity {
                self.velocity.0 = self.max_velocity;
            }

            // X 감속
            self.velocity.0 -= self.decelaration * dt;

            // Bound condition
            if self.velocity.0 < 0. {
                self.velocity.0 = 0.
            }
        } else if self.velocity.0 < 0. {
            // X 속도의 최대치 계산
            if self.velocity.0 < -self.max_velocity {
                self.velocity.0 = -self.max_velocity;
            }

            // X 감속
            self.velocity.0 += self.decelaration * dt;

            // Bound condition
            if self.velocity.0 > 0. {
                self.velocity.0 = 0.
            }
        }

        if self.velocity.1 > 0. {
            // Y 속도의 최대치 계산
            if self.velocity.1 > self.max_velocity {
                self.velocity.1 = self.max_velocity;
            }

            // Y 감속
            self.velocity.1 -= self.decelaration * dt;

            // Bound condition
            if self.velocity.1 < 0. {
                self.velocity.1 = 0.
            }
        } else if self.velocity.1 < 0. {
            // Y 속도의 최대치 계산
            if self.velocity.1 < -self.max_velocity {
                self.velocity.1 = -self.max_velocity;
            }

            // Y 감속
            self.velocity.1 += self.decelaration * dt;

            // Bound condition
            if self.velocity.1 > 0. {
                self.velocity.1 = 0.
            }
        }
    }

    pub fn get_predict_y(&self, dt: f64) -> f64 {
        // 현재 속도상의 다음 y 위치를 구한다.
        let predict_y = self.y + self.velocity.1 * dt;
        predict_y.min(WORLD_HEIGHT as f64).max(0.0)
    }

    pub fn get_predict_x(&self, dt: f64) -> f64 {
        // 현재 속도상의 다음 x 위치를 구한다.
        let predict_x = self.x + self.velocity.0 * dt;
        predict_x.min(WORLD_WIDTH as f64).max(0.0)
    }

    pub fn update_predict(&mut self, dt: f64) {
        self.update_velocity(dt);

        self.px = self.x;
        self.py = self.y;

        // x, y 이동 단위 이동속도를 구했다면 px, py에 해당 값을 더한다.
        self.px += self.velocity.0 * dt;
        self.py += self.velocity.1 * dt;

        // x, y에 대한 Bound Condition
        self.px = (self.px).min(WORLD_WIDTH as f64).max(0.0);
        self.py = (self.py).min(WORLD_HEIGHT as f64).max(0.0);
    }

    /// 해당 캐릭터의 x 속도를 0으로 리셋한다.
    /// 속도를 리셋하면서 px도 리셋한다.
    pub fn reset_velocity_x(&mut self) {
        self.velocity.0 = 0.0;
        self.px = self.x;
    }

    /// 해당 캐릭터의 y 속도를 0으로 리셋한다.
    /// 속도를 리셋하면서 py도 리셋한다.
    pub fn reset_velocity_y(&mut self) {
        self.velocity.1 = 0.0;
        self.py = self.y;
    }
    /// velocity를 재설정
    pub fn set_velocity(&mut self, v: Vector2<f64>) {
        self.velocity.0 = v.0;
        if v.0 == 0.0 {
            self.px = self.x;
        }

        self.velocity.1 = v.1;
        if v.1 == 0.0 {
            self.py = self.y;
        }
    }

    /// 캐릭터의 이동 벡터를 설정한다.
    pub fn move_forward(&mut self, direction: (f64, f64), dt: f64) {
        self.velocity.0 += direction.0 * self.accelaration * dt;
        self.velocity.1 += direction.1 * self.accelaration * dt;
    }

    /// 해당 캐릭터를 움직이게한다.
    pub fn update(&mut self, _dt: f64) {
        // 먼저 계산한 px, py 값을 새로운 x, y값으로 전환한다.

        self.x = self.px;
        self.y = self.py;
    }

    /// 방향 벡터를 설정한다.
    pub fn set_facing(&mut self, facing: Vector2<i32>) {
        self.facing = facing;
    }

    /// x, y위치를 동시에 설정한다.
    pub fn set_pos(&mut self, pos: (f64, f64)) {
        self.x = pos.0;
        self.y = pos.1;
    }

    /// x 위치를 설정한다.
    pub fn set_pos_x(&mut self, x: f64) {
        self.x = x;
    }

    /// y 위치를 설정한다.
    pub fn set_pos_y(&mut self, y: f64) {
        self.y = y;
    }

    /// x, y위치를 동시에 가져온다.
    pub fn get_pos(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    /// x 위치를 가져온다.
    pub fn get_pos_x(&self) -> f64 {
        self.x
    }

    /// y위치를 가져온다.
    pub fn get_pos_y(&self) -> f64 {
        self.y
    }

    /// facing 정보를 가져온다.
    pub fn get_facing(&self) -> Vector2<i32> {
        self.facing
    }
}
