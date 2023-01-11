/// 사각 그리드
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Grid {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

/// 생성 및 AABB
impl Grid {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Grid {
        Grid { x, y, w, h }
    }

    // aabb collision
    pub fn aabb(&self, other: &Grid) -> bool {
        self.x < (other.x + other.w as i32)
            && (self.x + self.w as i32) > other.x
            && self.y < (other.y + other.h as i32)
            && (self.y + self.h as i32) > other.y
    }
}
