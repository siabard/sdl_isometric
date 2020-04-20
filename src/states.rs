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
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::video::WindowContext;

use sdl2::mixer::Chunk;
use sdl2::mixer::Music;

pub trait States {
    ///  키 입력 등 일반적인 부분의 처리
    fn process_event(&mut self, event: &sdl2::event::Event) -> StateResult;

    /// 마우스 입력부분만 여기서 처리
    fn process_mouse(
        &mut self,
        x: i32,
        y: i32,
        new_buttons: &HashSet<sdl2::mouse::MouseButton>,
        old_buttons: &HashSet<sdl2::mouse::MouseButton>,
    );

    /// state 값을 변경시키는 부분에 대한 처리
    fn update(&mut self, dt: f64) -> StateResult;

    /// 화면에 노출시키기
    fn render(&self, canvas: &mut WindowCanvas) -> StateResult;

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
    fn process_event(&mut self, event: &sdl2::event::Event) -> StateResult {
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

    fn render(&self, canvas: &mut WindowCanvas) -> StateResult {
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
    unit_char: HashMap<Direction, Rc<RefCell<UnitCharacter>>>,
    music: Option<Music<'a>>,
    chunks: HashMap<String, Rc<RefCell<Chunk>>>,
    state_result: StateResult,
    map: Option<Map>,
    direction: Direction,
}

impl<'a> GameState<'a> {
    pub fn new() -> GameState<'a> {
        let texture_manager = TextureManager::new();
        GameState {
            texture_manager,
            unit_char: HashMap::new(),
            state_result: StateResult::Default,
            map: None,
            music: None,
            chunks: HashMap::new(),
            direction: Direction::Down,
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
        let mut uc: UnitCharacter = UnitCharacter::new(w, h, max_frame, fliph, flipv);
        let mut uc_vec = vec![];

        for i in 0..max_frame {
            uc_vec.push(Rect::new(x + i as i32 * w as i32, y, w, h));
        }
        uc.set_animation(uc_vec);
        self.unit_char.insert(id, Rc::new(RefCell::new(uc)));
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
            String::from(Character::PLAYER),
            "resources/GodotPlayer.png".to_string(),
        );

        // 캐릭터 애니메이션 생성
        self.add_unit_char(Direction::Down, 0, 0, 16, 16, 2, false, false);
        self.add_unit_char(Direction::Left, 32, 0, 16, 16, 2, true, false);
        self.add_unit_char(Direction::Up, 64, 0, 16, 16, 2, false, false);
        self.add_unit_char(Direction::Right, 32, 0, 16, 16, 2, false, false);

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

        // 음원 등록
        self.add_music("resources/beat.wav".to_owned());

        self.add_sound("high".to_owned(), "resources/high.wav".to_owned());
        self.add_sound("low".to_owned(), "resources/low.wav".to_owned());
    }
}

impl<'a> States for GameState<'a> {
    fn process_event(&mut self, event: &sdl2::event::Event) -> StateResult {
        self.state_result = match event {
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                self.direction = Direction::Up;
                StateResult::Default
            }

            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => {
                self.direction = Direction::Down;
                StateResult::Default
            }
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                self.direction = Direction::Left;
                StateResult::Default
            }

            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                self.direction = Direction::Right;
                StateResult::Default
            }

            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => StateResult::Pop,
            Event::KeyDown {
                keycode: Some(Keycode::Num1),
                ..
            } => {
                let chunk = self.chunks.get(&"high".to_owned()).unwrap().borrow();
                sdl2::mixer::Channel::all().play(&chunk, 0).unwrap();
                return StateResult::Default;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Num2),
                ..
            } => {
                let chunk = self.chunks.get(&"low".to_owned()).unwrap().borrow();
                sdl2::mixer::Channel::all().play(&chunk, 0).unwrap();
                return StateResult::Default;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Num0),
                ..
            } => {
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
                return StateResult::Default;
            }
            _ => StateResult::Default,
        };

        StateResult::Default
    }

    fn update(&mut self, dt: f64) -> StateResult {
        let unit_char_refcell = self.unit_char.get(&self.direction).unwrap();
        let mut unit_char = unit_char_refcell.borrow_mut();

        unit_char.update(dt);

        StateResult::Default
    }
    fn render(&self, canvas: &mut WindowCanvas) -> StateResult {
        // map 먼저 출력
        if let Some(map) = &self.map {
            map.render(canvas, &self.texture_manager);
        }
        // 모든 스프라이트를 WindowCanvas 에 출력..
        // 다 좋은데... x,y 좌표는??
        let texture_refcell = self
            .texture_manager
            .textures
            .get(&String::from(Character::PLAYER))
            .unwrap();
        let texture = texture_refcell.borrow();

        let unit_char_refcell = self.unit_char.get(&self.direction).unwrap();
        let unit_char = unit_char_refcell.borrow();

        unit_char.render(canvas, &texture);
        StateResult::Default
    }

    fn process_mouse(
        &mut self,
        x: i32,
        y: i32,
        new_buttons: &HashSet<sdl2::mouse::MouseButton>,
        old_buttons: &HashSet<sdl2::mouse::MouseButton>,
    ) {
        if !new_buttons.is_empty() || !old_buttons.is_empty() {
            let v_x = transform_value(x, REVERSE_WIDTH_RATIO);
            let v_y = transform_value(y, REVERSE_HEIGHT_RATIO);

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
