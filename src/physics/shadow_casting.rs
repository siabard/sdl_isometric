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
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub visible: Vec<bool>, // 해당 셀이 보이는가
    pub walled: Vec<bool>,  // 해당 셀이 벽인가
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

    // 벽 정보를 초기화한다.
    pub fn clear_wall(&mut self) {
        self.walled.fill(false);
    }

    // 보이는 정보를 모두 초기화한다.
    pub fn clear_visible(&mut self) {
        self.visible.fill(false);
    }

    // 해당 위치를 벽으로 선언한다.
    pub fn set_wall(&mut self, pos: Pos) {
        if pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.width && pos.1 < self.height {
            let idx = (pos.1 * self.width + pos.0) as usize;

            if let Some(w) = self.walled.get_mut(idx) {
                *w = true;
            }
        }
    }

    // 해당 위치를 보임으로 전환한다.
    pub fn reveal(&mut self, pos: Pos) {
        if pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.width && pos.1 < self.height {
            let idx = (pos.1 * self.width + pos.0) as usize;

            if let Some(v) = self.visible.get_mut(idx) {
                *v = true;
            }
        }
    }

    pub fn calculate_pov(&mut self, depth: i32, origin: Pos) {
        // origin 은 일단 보여야함
        self.reveal(origin);

        let row_north: Row = Row::new(1, 1., -1.);
        let row_south: Row = Row::new(1, -1., 1.);
        let row_east: Row = Row::new(1, -1., 1.);
        let row_west: Row = Row::new(1, 1., -1.);

        self.scan(Direction::North, origin, &row_north);
        self.scan(Direction::South, origin, &row_south);
        self.scan(Direction::East, origin, &row_east);
        self.scan(Direction::West, origin, &row_west);
    }

    /// is_floor
    pub fn is_floor(&self, tile: &Option<Pos>) -> bool {
        match tile {
            Some(pos) => {
                let idx = (pos.1 * self.width + pos.0) as usize;
                if idx >= (self.width * self.height) as usize {
                    false
                } else {
                    (*self.walled.get(idx).unwrap()) == false
                }
            }
            _ => false,
        }
    }

    /// is_wall
    pub fn is_wall(&self, tile: &Option<Pos>) -> bool {
        match tile {
            Some(_) => !self.is_floor(tile),
            _ => false,
        }
    }

    /// is_visible
    pub fn is_visible(&self, tile: &Option<Pos>) -> bool {
        match tile {
            Some(pos) => {
                let idx = (pos.1 * self.width + pos.0) as usize;
                if idx >= (self.width * self.height) as usize {
                    false
                } else {
                    (*self.visible.get(idx).unwrap())
                }
            }
            _ => false,
        }
    }

    pub fn scan(&mut self, direction: Direction, origin: Pos, old_row: &Row) {
        // row를 우선 복제한다.
        let mut row = Row::new(old_row.depth, old_row.start_slope, old_row.end_slope);

        // 최대 반경 4
        if row.depth > 4 {
            return;
        }

        // 스캔 위치를 먼저 구하자.
        let start_slope = row.start_slope;
        let end_slope = row.end_slope;

        // NEWS 에 따른 시작, 종료 위치
        let tiles: Vec<Pos> = row.tiles(origin, direction);

        let mut prev_tile: Option<Pos> = None;

        for tile in tiles.iter() {
            // tile 위치가 벗어나면 해당 tile은 검사안함
            if tile.0 < 0 || tile.1 < 0 || tile.0 >= self.width || tile.1 >= self.height {
                continue;
            }

            let idx = (tile.1 * self.width + tile.0) as usize;
            let is_visible = is_symmetric(&row, direction, origin, *tile);

            if self.is_wall(&Some(*tile)) || is_visible {
                self.reveal(*tile);
            }

            if self.is_wall(&prev_tile) && self.is_floor(&Some(*tile)) {
                let old_start_slope = row.start_slope;

                row.start_slope = slope(direction, origin, *tile);
            }

            if self.is_floor(&prev_tile) && self.is_wall(&Some(*tile)) {
                let old_end_slope = row.end_slope;
                let mut next_row = row.next();
                next_row.end_slope = slope(direction, origin, *tile);

                self.scan(direction, origin, &next_row);
            }

            prev_tile = Some(*tile);
        }

        if self.is_floor(&prev_tile) {
            let next_row = row.next();
            self.scan(direction, origin, &next_row);
        }
    }
}

/// 기울기 구하기
fn slope(direction: Direction, pos1: Pos, pos2: Pos) -> f32 {
    match direction {
        Direction::East | Direction::West => {
            ((pos2.1 - pos1.1) as f32 * 2.0 - 1.0) / ((pos2.0 - pos1.0) as f32 * 2.0)
        }
        Direction::North | Direction::South => {
            (pos2.1 - pos1.1) as f32 * 2.0 / ((pos2.0 - pos1.0) as f32 * 2.0 - 1.0)
        }
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

        if direction == Direction::East || direction == Direction::West {
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

/// `is_symmetric` checks if a given floor tile can be seen
/// symmetrically from the origin. It returns true if the central
/// point of the tile is in the sector swept out by the row’s start
/// and end slopes. Otherwise, it returns false.
fn is_symmetric(row: &Row, direction: Direction, origin: Pos, pos: Pos) -> bool {
    let depth = row.depth;
    let start_slope = row.start_slope;
    let end_slope = row.end_slope;

    match direction {
        Direction::North => {
            let lower_boundary = origin.0 - (row.depth as f32 / row.start_slope) as i32;
            let upper_boundary = origin.0 - (row.depth as f32 / row.end_slope) as i32;

            pos.0 >= lower_boundary && pos.0 <= upper_boundary
        }
        Direction::South => {
            let lower_boundary = origin.0 + (row.depth as f32 / row.start_slope) as i32;
            let upper_boundary = origin.0 + (row.depth as f32 / row.end_slope) as i32;

            pos.0 >= lower_boundary && pos.0 <= upper_boundary
        }
        Direction::East => {
            let lower_boundary = origin.1 + (row.depth as f32 * row.start_slope) as i32;
            let upper_boundary = origin.1 + (row.depth as f32 * row.end_slope) as i32;

            pos.1 >= lower_boundary && pos.1 <= upper_boundary
        }
        Direction::West => {
            let lower_boundary = origin.1 - (row.depth as f32 * row.start_slope) as i32;
            let upper_boundary = origin.1 - (row.depth as f32 * row.end_slope) as i32;

            pos.1 >= lower_boundary && pos.1 <= upper_boundary
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
            let start_pos_x = origin.0 - ((depth as f32 / start_slope + 0.5).floor() as i32);
            let start_pos_y = origin.1 - depth;
            let end_pos_x = origin.0 - ((depth as f32 / end_slope - 0.5).ceil() as i32);
            let end_pos_y = origin.1 - depth;

            ((start_pos_x, start_pos_y), (end_pos_x, end_pos_y))
        }

        Direction::South => {
            let start_pos_x = origin.0 + ((depth as f32 / start_slope + 0.5).floor() as i32);
            let start_pos_y = origin.1 + depth;
            let end_pos_x = origin.0 + ((depth as f32 / end_slope - 0.5).ceil() as i32);
            let end_pos_y = origin.1 + depth;

            ((start_pos_x, start_pos_y), (end_pos_x, end_pos_y))
        }
        Direction::East => {
            let start_pos_x = origin.0 + depth;
            let start_pos_y = origin.1 + ((depth as f32 * start_slope + 0.5).floor() as i32);
            let end_pos_x = origin.0 + depth;
            let end_pos_y = origin.1 + ((depth as f32 * end_slope - 0.5).ceil() as i32);

            ((start_pos_x, start_pos_y), (end_pos_x, end_pos_y))
        }
        Direction::West => {
            let start_pos_x = origin.0 - depth;
            let start_pos_y = origin.1 - ((depth as f32 * start_slope + 0.5).floor() as i32);
            let end_pos_x = origin.0 - depth;
            let end_pos_y = origin.1 - ((depth as f32 * end_slope - 0.5).ceil() as i32);

            ((start_pos_x, start_pos_y), (end_pos_x, end_pos_y))
        }
    }
}
