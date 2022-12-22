//! 아스키로 만드는 맵

const DEG2RAD: f32 = 3.14159 / 180.;

/// 거리 측정
fn dist_diag(x0: i32, y0: i32, x1: i32, y1: i32) -> i32 {
    (x1 - x0).abs().max((y1 - y0).abs())
}

/// 선형 Tweening
fn lerpl(start: f32, end: f32, t: f32) -> f32 {
    start + t * (end - start)
}

#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub struct Item {
    id: u32,
    name: String,
}

/// 지도의 각 셀은 벽이나 바닥
/// 바닥에는 복수의 아이템이 놓일 수 있다.
#[derive(Clone, PartialEq)]
pub enum MapCell {
    Blank,
    Wall,
    Floor(Vec<Item>),
}

pub struct Map {
    pub cells: Vec<MapCell>,
    pub glared: Vec<bool>,
    pub known: Vec<bool>,
    pub width: u32,
    pub height: u32,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        let mut cells = vec![];
        for i in 0..(width * height) {
            cells.push(MapCell::Blank);
        }

        Map {
            cells,
            glared: [false].repeat((width * height) as usize),
            known: [false].repeat((width * height) as usize),
            width,
            height,
        }
    }

    /// FOV에 따라 맵을 변환한다.
    /// https://www.youtube.com/watch?v=jHep5d3gXt8 을 포팅했다.
    pub fn fov(&mut self, x: i32, y: i32, dist: u32) {
        // 전체를 안보이는 것으로 설정한다.
        for i in 0..(self.width * self.height) {
            self.glared[i as usize] = false;
        }

        // x, y를 중심으로 360도 사방으로 빛을 쏘아, 벽에 닿을 때까지
        // 보이게 한다.
        for i in 0..360 {
            let deg: f32 = i as f32 * DEG2RAD;

            let x_: i32 = (deg.cos() * (dist as f32)).round() as i32 + x;
            let y_: i32 = (deg.sin() * (dist as f32)).round() as i32 + y;

            let d: i32 = dist_diag(x, y, x_, y_);

            for j in 0..d {
                let tx: i32 = lerpl(x as f32, x_ as f32, j as f32 / d as f32) as i32;
                let ty: i32 = lerpl(y as f32, y_ as f32, j as f32 / d as f32) as i32;

                if tx < 0 || tx >= self.width as i32 {
                    continue;
                }

                if ty < 0 || ty >= self.height as i32 {
                    continue;
                }

                // 중심점에서부터 빛이 나아가는 경로에 벽이 있다면
                // 해당 벽까지만 빛이 들어가야하고 그 이후로는 더이상 진행해서는 안된다.
                let idx: usize = (tx + ty * self.width as i32) as usize;
                self.known[idx] = true;
                if self.cells[idx] == MapCell::Wall {
                    break;
                }
                self.glared[idx] = true;
            }
        }
    }
}
