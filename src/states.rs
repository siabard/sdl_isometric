use crate::animation::*;
use crate::constant::*;
use crate::gui::*;
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
    fn update(&mut self) -> StateResult;

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
    texture: Option<Texture<'a>>,
    buttons: HashMap<String, Rc<RefCell<GuiElement<'a>>>>,
    state_result: StateResult,
}

impl<'a> InitState<'a> {
    pub fn new() -> InitState<'a> {
        InitState {
            texture: None,
            buttons: HashMap::new(),
            state_result: StateResult::Default,
        }
    }

    pub fn init(
        &mut self,
        texture_creator: &'a TextureCreator<WindowContext>,
        font_context: &'a sdl2::ttf::Sdl2TtfContext,
    ) {
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

        self.texture = Some(texture);

        let start_button_uuid = Uuid::new_v4();
        let start_button = GuiElement::new(
            start_button_uuid,
            texture_creator,
            &Path::new("resources/btn_normal.png"),
            &Path::new("resources/btn_hover.png"),
            10,
            10,
            32,
            32,
        );

        self.buttons.insert(
            "start_button".to_owned(),
            Rc::new(RefCell::new(start_button)),
        );
    }
}

impl<'a> States for InitState<'a> {
    fn process_event(&mut self, event: &sdl2::event::Event) -> StateResult {
        for (_k, v) in self.buttons.iter_mut() {
            let mut button = v.borrow_mut();
            button.process_event(&event);
        }

        StateResult::Default
    }

    fn update(&mut self) -> StateResult {
        // 화면의 모든 버튼에 대한 update
        for (_k, v) in self.buttons.iter_mut() {
            let mut button = v.borrow_mut();
            button.update();
        }

        // start_button이 클릭되었다면 GameState로 이동해야한다.
        let start_button_refcell = self.buttons.get(&"start_button".to_owned()).unwrap();

        let mut button = start_button_refcell.borrow_mut();
        if button.is_clicked {
            button.reset();
            self.state_result = StateResult::Push(StateInfo::Game("stage_1"))
        } else {
            self.state_result = StateResult::Default
        }

        StateResult::Default
    }

    fn render(&self, canvas: &mut WindowCanvas) -> StateResult {
        // 화면의 모든 버튼을 출력하기
        for (_k, v) in self.buttons.iter() {
            let button = v.borrow();
            button.render(canvas);
        }

        // 화면에 텍스트를 출력하기
        canvas
            .copy_ex(
                self.texture.as_ref().unwrap(),
                None,
                Some(transform_rect(
                    &Rect::new(0, 0, VIRTUAL_WIDTH, VIRTUAL_HEIGHT),
                    WIDTH_RATIO,
                    HEIGHT_RATIO,
                )),
                0.,
                Some(Point::new(0, 0)),
                false,
                false,
            )
            .unwrap();

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
        for (_k, v) in self.buttons.iter_mut() {
            let mut button = v.borrow_mut();
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

pub struct GameState<'a> {
    sprites: HashMap<Character, Rc<RefCell<Texture<'a>>>>,
    unit_char: HashMap<Direction, Rc<RefCell<UnitCharacter>>>,
    state_result: StateResult,
}

impl<'a> GameState<'a> {
    pub fn new() -> GameState<'a> {
        let sprites = HashMap::new();

        GameState {
            sprites,
            unit_char: HashMap::new(),
            state_result: StateResult::Default,
        }
    }

    pub fn add_sprite(
        &mut self,
        creator: &'a TextureCreator<WindowContext>,
        key: Character,
        path: String,
    ) {
        let texture = creator.load_texture(Path::new(&path)).unwrap();
        self.sprites.insert(key, Rc::new(RefCell::new(texture)));
    }

    pub fn add_unit_char(&mut self, id: Direction, w: u32, h: u32, max_frame: u32) {
        let mut uc: UnitCharacter = UnitCharacter::new(w, h, max_frame);
        let mut uc_vec = vec![];

        for i in 0..max_frame {
            uc_vec.push(Rect::new(i as i32 * w as i32, 0, w, h));
        }
        uc.set_animation(uc_vec);
        self.unit_char.insert(id, Rc::new(RefCell::new(uc)));
    }

    pub fn init(
        &mut self,
        texture_creator: &'a TextureCreator<WindowContext>,
        _font_context: &'a sdl2::ttf::Sdl2TtfContext,
    ) {
        self.add_sprite(
            texture_creator,
            Character::Player,
            "resources/GodotPlayer.png".to_string(),
        );

        self.add_unit_char(Direction::Left, 16, 16, 4);
    }
}

impl<'a> States for GameState<'a> {
    fn process_event(&mut self, event: &sdl2::event::Event) -> StateResult {
        self.state_result = match event {
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => StateResult::Pop,
            _ => StateResult::Default,
        };

        StateResult::Default
    }

    fn update(&mut self) -> StateResult {
        let unit_char_refcell = self.unit_char.get(&Direction::Left).unwrap();
        let mut unit_char = unit_char_refcell.borrow_mut();

        unit_char.update();

        StateResult::Default
    }
    fn render(&self, canvas: &mut WindowCanvas) -> StateResult {
        // 모든 스프라이트를 WindowCanvas 에 출력..
        // 다 좋은데... x,y 좌표는??
        let texture_refcell = self.sprites.get(&Character::Player).unwrap();
        let texture = texture_refcell.borrow();

        let unit_char_refcell = self.unit_char.get(&Direction::Left).unwrap();
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
