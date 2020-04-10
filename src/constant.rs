// 이건 실제 노출되는 물리적인 화면의 크기이다.
pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;

// 논리적인 게임데이터는 VIRTUAL_* 환경에서 돌아가는 것으로 가정한다.
pub const VIRTUAL_WIDTH: u32 = 400;
pub const VIRTUAL_HEIGHT: u32 = 300;

pub const WIDTH_RATIO: f32 = SCREEN_WIDTH as f32 / VIRTUAL_WIDTH as f32;
pub const HEIGHT_RATIO: f32 = SCREEN_HEIGHT as f32 / VIRTUAL_HEIGHT as f32;

pub const REVERSE_WIDTH_RATIO: f32 = 1.0 / WIDTH_RATIO;
pub const REVERSE_HEIGHT_RATIO: f32 = 1.0 / HEIGHT_RATIO;
