use crate::animation::*;
use crate::constant::*;
use crate::gui::*;
use crate::map::*;
use crate::texture_manager::*;
use crate::*;

use uuid::Uuid;

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;

use std::path::Path;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::video::WindowContext;

use sdl2::mixer::Chunk;
use sdl2::mixer::Music;

pub trait States {
    ///  키 입력 등 일반적인 부분의 처리
    fn process_event(&mut self, event: &sdl2::event::Event, dt: f64) -> StateResult;

    /// 마우스 입력부분만 여기서 처리
    fn process_mouse(
        &mut self,
        x: i32,
        y: i32,
        new_buttons: &HashSet<sdl2::mouse::MouseButton>,
        old_buttons: &HashSet<sdl2::mouse::MouseButton>,
        dt: f64,
    );

    /// state 값을 변경시키는 부분에 대한 처리
    fn update(&mut self, dt: f64) -> StateResult;

    /// 화면에 노출시키기
    fn render(&mut self, canvas: &mut WindowCanvas) -> StateResult;

    /// main loop에서 States의 다음 상태를 요청할 때
    fn next_result(&mut self) -> StateResult;
}

/// 초기 상태
/// 메뉴 처리
pub struct InitState<'a> {
    //font: sdl2::ttf::Font<'a, 'b>,
    //surface: sdl2::surface::Surface<'a>,
    texture_manager: Option<TextureManager<'a>>,
    buttons: HashMap<String, GuiElement>,
    state_result: StateResult,
}

impl<'a> InitState<'a> {
    pub fn new() -> InitState<'a> {
        InitState {
            texture_manager: None,
            buttons: HashMap::new(),
            state_result: StateResult::Default,
        }
    }

    pub fn init(
        &mut self,
        texture_creator: &'a TextureCreator<WindowContext>,
        font_context: &'a sdl2::ttf::Sdl2TtfContext,
    ) {
        self.texture_manager = Some(TextureManager::new());

        let font = font_context
            .load_font(Path::new("resources/hackr.ttf"), 36)
            .unwrap();

        let surface = font
            .render("Init State")
            .blended(Color::RGBA(255, 0, 0, 255))
            .unwrap();

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        let texture_manager = self.texture_manager.as_mut().unwrap();

        texture_manager.add_texture("text".to_owned(), Rc::new(RefCell::new(texture)));

        texture_manager.load_texture(
            "normal_button".to_string(),
            texture_creator,
            &Path::new("resources/btn_normal.png"),
        );
        texture_manager.load_texture(
            "hover_button".to_string(),
            texture_creator,
            &Path::new("resources/btn_hover.png"),
        );

        let text = GuiElement::new(
            Uuid::new_v4(),
            ("text".to_owned(), "init_state".to_string()),
            ("text".to_owned(), "init_state".to_string()),
            0,
            0,
            surface.width(),
            surface.height(),
        );

        self.buttons.insert("text".to_owned(), text);

        let start_button_uuid = Uuid::new_v4();
        let start_button = GuiElement::new(
            start_button_uuid,
            ("normal_button".to_string(), "normal_button".to_string()),
            ("hover_button".to_string(), "hover_button".to_string()),
            100,
            100,
            32,
            32,
        );

        self.buttons.insert("start_button".to_owned(), start_button);
    }
}

impl<'a> States for InitState<'a> {
    fn process_event(&mut self, event: &sdl2::event::Event, _dt: f64) -> StateResult {
        for (_k, button) in self.buttons.iter_mut() {
            button.process_event(&event);
        }
        StateResult::Default
    }

    fn update(&mut self, _dt: f64) -> StateResult {
        // 화면의 모든 버튼에 대한 update
        for (_k, button) in self.buttons.iter_mut() {
            button.update();
        }

        // start_button이 클릭되었다면 GameState로 이동해야한다.
        let start_button = self.buttons.get_mut(&"start_button".to_owned()).unwrap();

        if start_button.is_clicked {
            start_button.reset();
            self.state_result = StateResult::Push(StateInfo::Game("stage_1"))
        } else {
            self.state_result = StateResult::Default
        }

        StateResult::Default
    }

    fn render(&mut self, canvas: &mut WindowCanvas) -> StateResult {
        // 화면의 모든 GUI요소를 출력하기
        for (_k, button) in self.buttons.iter() {
            button.render(canvas, self.texture_manager.as_ref().unwrap());
        }

        StateResult::Default
    }

    fn process_mouse(
        &mut self,
        x: i32,
        y: i32,
        new_buttons: &HashSet<sdl2::mouse::MouseButton>,
        old_buttons: &HashSet<sdl2::mouse::MouseButton>,
        _dt: f64,
    ) {
        // 화면의 버튼을 이용
        for (_k, button) in self.buttons.iter_mut() {
            button.process_mouse(x, y, new_buttons, old_buttons);
        }

        // 물리적인 좌표를 가상위치값으로 바꾼다.

        /*
        let v_x = transform_value(x, REVERSE_WIDTH_RATIO);
        let v_y = transform_value(y, REVERSE_WIDTH_RATIO);
        if !new_buttons.is_empty() || !old_buttons.is_empty() {
            println!(
                "X = {:?}, Y = {:?} : {:?} -> {:?}",
                v_x, v_y, new_buttons, old_buttons
            );
        }
        */
    }

    fn next_result(&mut self) -> StateResult {
        let result = self.state_result;
        self.state_result = StateResult::Default;

        result
    }
}

/// 게임 실행용 State
pub struct GameState<'a> {
    texture_manager: TextureManager<'a>,
    pc: UnitCharacter,
    enemy: UnitCharacter,
    music: Option<Music<'a>>,
    chunks: HashMap<String, Rc<RefCell<Chunk>>>,
    state_result: StateResult,
    map: Option<Map>,
    keyboards: HashSet<sdl2::keyboard::Keycode>,
    cx: i32, // 카메라 X 좌표
    cy: i32, // 카메라 Y 좌표
    cw: u32, // 카메라 폭
    ch: u32, // 카메라 높이
}

impl<'a> GameState<'a> {
    pub fn new() -> GameState<'a> {
        let texture_manager = TextureManager::new();
        let pc = UnitCharacter::new(16, 16, 2, 200., 1500., 900.);

        let enemy = UnitCharacter::new(16, 16, 2, 200., 1500., 900.);
        GameState {
            texture_manager,
            pc,
            enemy,
            state_result: StateResult::Default,
            map: None,
            music: None,
            chunks: HashMap::new(),
            keyboards: HashSet::new(),
            cx: 0,
            cy: 0,
            cw: VIRTUAL_WIDTH,
            ch: VIRTUAL_HEIGHT,
        }
    }

    pub fn add_texture(
        &mut self,
        texture_creator: &'a TextureCreator<WindowContext>,
        key: String,
        path: String,
    ) {
        self.texture_manager
            .load_texture(key, texture_creator, Path::new(&path));
    }

    pub fn add_unit_char(
        &mut self,
        id: Direction,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        max_frame: u32,
        fliph: bool,
        flipv: bool,
    ) {
        let mut uc_vec = vec![];
        for i in 0..max_frame {
            uc_vec.push(Rect::new(x + i as i32 * w as i32, y, w, h));
        }
        self.pc.set_animation(id, uc_vec.clone(), fliph, flipv);
        self.enemy.set_animation(id, uc_vec, fliph, flipv);
    }

    pub fn add_music(&mut self, path: String) {
        let music = sdl2::mixer::Music::from_file(&Path::new(&path)).unwrap();
        self.music = Some(music);
    }

    pub fn add_sound(&mut self, key: String, path: String) {
        let chunk = sdl2::mixer::Chunk::from_file(&Path::new(&path)).unwrap();

        self.chunks.insert(key, Rc::new(RefCell::new(chunk)));
    }

    pub fn init(
        &mut self,
        texture_creator: &'a TextureCreator<WindowContext>,
        _font_context: &'a sdl2::ttf::Sdl2TtfContext,
    ) {
        self.add_texture(
            texture_creator,
            String::from(character::PLAYER),
            "resources/GodotPlayer.png".to_string(),
        );

        // 캐릭터 애니메이션 생성
        self.add_unit_char(Direction::Down, 0, 0, 16, 16, 2, false, false);
        self.add_unit_char(Direction::Left, 32, 0, 16, 16, 2, true, false);
        self.add_unit_char(Direction::Up, 64, 0, 16, 16, 2, false, false);
        self.add_unit_char(Direction::Right, 32, 0, 16, 16, 2, false, false);

        self.pc.set_hitbox(2, 0, 12, 16);
        self.enemy.set_hitbox(2, 0, 12, 16);

        // 지도 등록
        self.add_texture(
            texture_creator,
            "map".to_string(),
            "resources/map.png".to_string(),
        );
        let mut map = Map::new("map".to_owned(), 16, 16);
        map.load_map();
        map.init_map(0, 0, 0, 16, 16);
        map.init_map(1, 16, 0, 16, 16);
        map.init_map(2, 32, 0, 16, 16);
        self.map = Some(map);

        // 적 위치 변경
        self.enemy.x = 300.0;
        self.enemy.y = 200.0;

        // 음원 등록
        self.add_music("resources/beat.wav".to_owned());

        self.add_sound("high".to_owned(), "resources/high.wav".to_owned());
        self.add_sound("low".to_owned(), "resources/low.wav".to_owned());
    }
}

impl<'a> States for GameState<'a> {
    fn process_event(&mut self, event: &sdl2::event::Event, _dt: f64) -> StateResult {
        match event {
            Event::KeyDown {
                keycode: Some(k), ..
            } => {
                self.keyboards.insert(*k);
                if *k == Keycode::Num1 {
                    let chunk = self.chunks.get(&"high".to_owned()).unwrap().borrow();
                    sdl2::mixer::Channel::all().play(&chunk, 0).unwrap();
                } else if *k == Keycode::Num2 {
                    let chunk = self.chunks.get(&"low".to_owned()).unwrap().borrow();
                    sdl2::mixer::Channel::all().play(&chunk, 0).unwrap();
                } else if *k == Keycode::Num0 {
                    let music = self.music.as_ref().unwrap();

                    if sdl2::mixer::Music::is_playing() == false {
                        music.play(-1).unwrap();
                    } else {
                        if sdl2::mixer::Music::is_paused() {
                            sdl2::mixer::Music::resume();
                        } else {
                            sdl2::mixer::Music::pause();
                        }
                    }
                }

                if *k == Keycode::Escape {
                    self.state_result = StateResult::Pop;
                } else {
                    self.state_result = StateResult::Default;
                }
            }
            Event::KeyUp {
                keycode: Some(k), ..
            } => {
                self.keyboards.remove(&k);
                self.state_result = StateResult::Default;
            }
            _ => self.state_result = StateResult::Default,
        };

        StateResult::Default
    }

    fn update(&mut self, dt: f64) -> StateResult {
        // 키보드 처리
        if self.keyboards.contains(&Keycode::Up) || self.keyboards.contains(&Keycode::W) {
            self.pc.move_forward((0., -1.), dt);
        }
        if self.keyboards.contains(&Keycode::Down) || self.keyboards.contains(&Keycode::S) {
            self.pc.move_forward((0., 1.), dt);
        }
        if self.keyboards.contains(&Keycode::Left) || self.keyboards.contains(&Keycode::A) {
            self.pc.move_forward((-1., 0.), dt);
        }
        if self.keyboards.contains(&Keycode::Right) || self.keyboards.contains(&Keycode::D) {
            self.pc.move_forward((1., 0.), dt);
        }

        self.pc.update_predict(dt);
        self.enemy.update_predict(dt);

        // collision detection
        let directions = detect_collision(
            self.pc.get_hitbox_predict().as_ref().unwrap(),
            self.enemy.get_hitbox_predict().as_ref().unwrap(),
        );

        if directions.contains(&Direction::Down) {
            // 아래쪽이 닿은 경우
            // y백터가 0보다 크다면, y 벡터를 0으로 하고, py를 기존 y로 리셋
            if self.pc.velocity.1 > 0. {
                self.pc.reset_velocity_y();
                self.enemy.reset_velocity_y();
            }
        }

        if directions.contains(&Direction::Up) {
            // 위쪽이 닿은 경우
            // y벡터가 0보다 작으면, y 벡터를 0으로 하고, py를 기존 y로 리셋한다.
            if self.pc.velocity.1 < 0. {
                self.pc.reset_velocity_y();
                self.enemy.reset_velocity_y();
            }
        }
        if directions.contains(&Direction::Left) {
            // 왼쪽이 닿은 경우
            // x 벡터가 0보다 작다면,  x 벡터를 0으로 하고, px를 기존 x로 리셋한다.
            if self.pc.velocity.0 < 0. {
                self.pc.reset_velocity_x();
                self.enemy.reset_velocity_x();
            }
        }

        if directions.contains(&Direction::Right) {
            // 오른쪽이 닿은 경우
            // x 벡터가 0보다 크다면,  x 벡터를 0으로 하고, px를 기존 x로 리셋한다.
            if self.pc.velocity.0 > 0. {
                self.pc.reset_velocity_x();
                self.enemy.reset_velocity_x();
            }
        }

        // 실제 움직이게 한다.

        self.pc.update(dt);
        self.enemy.update(dt);
        StateResult::Default
    }

    fn render(&mut self, canvas: &mut WindowCanvas) -> StateResult {
        // cx, cy 를 기준으로 모든 좌표계를 이동해야한다.
        // 예를 들어, 현재 world 기준으로 (100,100)인데 (cx,cy)가 (100,100)이라면
        // 해당 좌표는 100,100만큼 작아져야한다.

        // cx, cy를 구한다.
        // cx, cy는 추적하는 캐릭터에 맞추어 정해진다.
        // 여기서는 unit_char이다.
        // cx + cw 구간 양쪽 10% 공간에 있다면 cx는 왼쪽으로는 10% 여백이 가능한 만큼 좌측으로 이동하고
        // 우측으로는 10% 여백이 가능한 만큼 우측으로 이동해야한다.
        // cy + ch 에 대해서도 동일한다.

        let ux = self.pc.x as i32;
        let uy = self.pc.y as i32;

        let width_margin = (self.cw as f32 * 0.1) as u32;
        let height_margin = (self.ch as f32 * 0.1) as u32;
        let left_limit = self.cx as u32 + width_margin;
        let right_limit = self.cx as u32 + self.cw - width_margin;
        let top_limit = self.cy as u32 + height_margin;
        let bottom_limit = self.cy as u32 + self.ch - height_margin as u32;

        if ux < left_limit as i32 {
            // cx를 ux 위치가 left_limit인 곳까지 이동한다.
            self.cx = ux - width_margin as i32;
            if self.cx < 0 {
                self.cx = 0;
            }
        } else if ux > right_limit as i32 {
            // cx를 ux 위치가 right_limit인 곳까지 이동한다.
            self.cx = ux - self.cw as i32 + width_margin as i32;
            if self.cx as u32 + self.cw > WORLD_WIDTH {
                self.cx = (WORLD_WIDTH - width_margin) as i32;
            }
        }

        if uy < top_limit as i32 {
            // cx를 ux 위치가 left_limit인 곳까지 이동한다.
            self.cy = uy - height_margin as i32;
            if self.cy < 0 {
                self.cy = 0;
            }
        } else if uy > bottom_limit as i32 {
            // cx를 ux 위치가 right_limit인 곳까지 이동한다.
            self.cy = uy - self.ch as i32 + height_margin as i32;
            if self.cy as u32 + self.ch > WORLD_HEIGHT {
                self.cy = (WORLD_HEIGHT - height_margin) as i32;
            }
        }

        let camera_rect = Rect::new(self.cx, self.cy, self.cw, self.ch);
        // map 먼저 출력
        if let Some(map) = &self.map {
            map.render(canvas, &camera_rect, &self.texture_manager);
        }
        // 모든 스프라이트를 WindowCanvas 에 출력..
        // 다 좋은데... x,y 좌표는??
        let texture_refcell = self
            .texture_manager
            .textures
            .get(&String::from(character::PLAYER))
            .unwrap();
        let texture = texture_refcell.borrow();

        self.pc.render(canvas, &camera_rect, &texture);
        self.enemy.render(canvas, &camera_rect, &texture);
        StateResult::Default
    }

    fn process_mouse(
        &mut self,
        x: i32,
        y: i32,
        new_buttons: &HashSet<sdl2::mouse::MouseButton>,
        old_buttons: &HashSet<sdl2::mouse::MouseButton>,
        _dt: f64,
    ) {
        let v_x = transform_value(x, REVERSE_WIDTH_RATIO);
        let v_y = transform_value(y, REVERSE_HEIGHT_RATIO);

        // 가상좌표에 따라 캐릭터의 바라보는 위치를 바꾼다.
        let diff_x = (self.pc.x - v_x as f32).abs();
        let diff_y = (self.pc.y - v_y as f32).abs();

        if diff_x > diff_y {
            if self.pc.x > v_x as f32 {
                self.pc.direction = Direction::Left;
            } else if self.pc.x < v_x as f32 {
                self.pc.direction = Direction::Right;
            }
        } else {
            if self.pc.y > v_y as f32 {
                self.pc.direction = Direction::Up;
            } else if self.pc.y < v_y as f32 {
                self.pc.direction = Direction::Down;
            }
        }

        self.pc.set_deg((v_x as f32, v_y as f32));
        if !new_buttons.is_empty() || !old_buttons.is_empty() {
            println!(
                "X = {:?}, Y = {:?} : {:?} -> {:?}",
                v_x, v_y, new_buttons, old_buttons
            );
        }
    }

    fn next_result(&mut self) -> StateResult {
        let result = self.state_result;
        self.state_result = StateResult::Default;

        result
    }
}
