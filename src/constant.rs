// 이건 실제 노출되는 물리적인 화면의 크기이다.
pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;

/// 게임이 진행되는 화면의 논리적 크기
/// 게임데이터는 VIRTUAL_* 환경에서 돌아가는 것으로 가정한다.
pub const VIRTUAL_WIDTH: u32 = 400;
pub const VIRTUAL_HEIGHT: u32 = 300;

/// 게임이 진행되는 전체 영역의 크기
/// VIRTUAL_WIDTH / VIRTUAL_HEIGHT 는 이 전체 영역에 대한
/// 일종의 CAMERA Viewport라고 보면 된다.
pub const WORLD_WIDTH: u32 = 4000;
pub const WORLD_HEIGHT: u32 = 3000;

pub const WIDTH_RATIO: f32 = SCREEN_WIDTH as f32 / VIRTUAL_WIDTH as f32;
pub const HEIGHT_RATIO: f32 = SCREEN_HEIGHT as f32 / VIRTUAL_HEIGHT as f32;

pub const REVERSE_WIDTH_RATIO: f32 = 1.0 / WIDTH_RATIO;
pub const REVERSE_HEIGHT_RATIO: f32 = 1.0 / HEIGHT_RATIO;

// delta t설정하기
pub const TIME_SPAN: u32 = 1_000_000_000 / 60;
pub const DELTA_T: f64 = 1.0f64 / 60.0;
