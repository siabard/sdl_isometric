//! Shadow Casting
//! 재귀반복을 이용해 Symmetric Shadowcasting 을 만든다.
//!
//! psuedo 코드는 아래와 같다.
//!
//! ```
//!  Scan(depth, startslope, endslope)
//!
//!    init y
//!    init x
//!
//!    while current_slope has not reached endslope do
//!      if (x,y) within visual range then
//!        if (x,y) blocked and prior not blocked then
//!          Scan(depth + 1, startslope, new_endslope)
//!        if (x,y) not blocked and prior blocked then
//!          new_startslope
//!        set (x,y) visible
//!      progress (x,y)
//!
//!    regress (x,y)
//!
//!    if depth < visual range and (x,y) not blocked
//!      Scan(depth + 1, startslope, endslope)
//!  end
//! ```
//!
//! 기본적으로 동서남북 4방향을 검증해야하고, 검증한 방향에 맞추어
//! 해당 셀에 대한 검증을 해야함

type Pos = (i32, i32);

/// 동서남북
#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// 가로 w, 세로 h의 2D공간에 대한 벡터를 처리하는 Scanner
/// 진행 가능한 distance 와 공간상의 위치도 필요
pub struct LightMap {
    pub width: i32,
    pub height: i32,
    pub visible: Vec<bool>,
    pub walled: Vec<bool>,
}

impl LightMap {
    pub fn new(width: i32, height: i32) -> LightMap {
        let mut visible: Vec<bool> = vec![];
        let mut walled: Vec<bool> = vec![];

        for _ in 0..(width * height) {
            visible.push(false);
            walled.push(false);
        }

        LightMap {
            width,
            height,
            visible,
            walled,
        }
    }

    pub fn calculate_pov(&mut self, depth: i32, origin: Pos) {
        // 4방향 모두에 대해 검사
    }

    pub fn scan(&mut self, direction: Direction, origin: Pos, row: &Row) {
        // 스캔 위치를 먼저 구하자.
        let start_slope = row.start_slope;
        let end_slope = row.end_slope;

        // NEWS 에 따른 시작, 종료 위치
        let tiles: Vec<Pos> = row.tiles(origin, direction);

        let prev_tile: Option<Pos> = None;

        dbg!(tiles);
    }
}

/// 각 행, 혹은 열에 대해서 각 단계별로 어디까지 검색해야하는지 알려주는 구조체
#[derive(Debug, Clone, Copy)]
pub struct Row {
    depth: i32,
    start_slope: f32,
    end_slope: f32,
}

impl Row {
    pub fn new(depth: i32, start_slope: f32, end_slope: f32) -> Row {
        Row {
            depth,
            start_slope,
            end_slope,
        }
    }

    pub fn next(&self) -> Row {
        // slope x depth 를 통해 row 정보를 제공한다.
        Row {
            depth: self.depth + 1,
            start_slope: self.start_slope,
            end_slope: self.end_slope,
        }
    }

    pub fn tiles(&self, origin: Pos, direction: Direction) -> Vec<Pos> {
        let (start_pos, end_pos) = get_pos(
            origin,
            self.depth,
            self.start_slope,
            self.end_slope,
            direction,
        );

        if start_pos.0 == end_pos.0 {
            (start_pos.1..=end_pos.1)
                .into_iter()
                .map(move |y| (start_pos.0, y))
                .collect()
        } else {
            (start_pos.0..=end_pos.0)
                .into_iter()
                .map(move |x| (x, start_pos.1))
                .collect()
        }
    }
}

fn get_pos(
    origin: Pos,
    depth: i32,
    start_slope: f32,
    end_slope: f32,
    direction: Direction,
) -> (Pos, Pos) {
    match direction {
        Direction::North => {
            let start_pos_x = origin.0 - ((depth as f32 / start_slope - 0.5).ceil() as i32);
            let start_pos_y = origin.1 - depth;
            let end_pos_x = origin.0 - ((depth as f32 / end_slope + 0.5).floor() as i32);
            let end_pos_y = origin.1 - depth;

            ((start_pos_x, start_pos_y), (end_pos_x, end_pos_y))
        }

        Direction::South => {
            let start_pos_x = origin.0 + ((depth as f32 / start_slope - 0.5).ceil() as i32);
            let start_pos_y = origin.1 + depth;
            let end_pos_x = origin.0 + ((depth as f32 / end_slope + 0.5).floor() as i32);
            let end_pos_y = origin.1 + depth;
            ((start_pos_x, start_pos_y), (end_pos_x, end_pos_y))
        }
        Direction::East => {
            let start_pos_x = origin.0 - depth;
            let start_pos_y = origin.1 - ((depth as f32 / start_slope - 0.5).ceil() as i32);
            let end_pos_x = origin.0 - depth;
            let end_pos_y = origin.1 - ((depth as f32 / end_slope + 0.5).floor() as i32);
            ((start_pos_x, start_pos_y), (end_pos_x, end_pos_y))
        }
        Direction::West => {
            let start_pos_x = origin.0 + depth;
            let start_pos_y = origin.1 + ((depth as f32 / start_slope - 0.5).ceil() as i32);
            let end_pos_x = origin.0 + depth;
            let end_pos_y = origin.1 + ((depth as f32 / end_slope + 0.5).floor() as i32);
            ((start_pos_x, start_pos_y), (end_pos_x, end_pos_y))
        }
    }
}
