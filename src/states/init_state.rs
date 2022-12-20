use crate::gui::*;
use crate::states::*;

use uuid::Uuid;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::collections::HashMap;
use std::collections::HashSet;

use std::path::Path;

use sdl2::render::WindowCanvas;
use sdl2::video::WindowContext;

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
            .render("Init State (press Q to quit)")
            .blended(Color::RGBA(255, 0, 0, 255))
            .unwrap();

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        let texture_manager = self.texture_manager.as_mut().unwrap();

        texture_manager.add_texture("text".to_owned(), texture);

        texture_manager.load_texture(
            "normal_button".to_string(),
            texture_creator,
            Path::new("resources/btn_normal.png"),
        );
        texture_manager.load_texture(
            "hover_button".to_string(),
            texture_creator,
            Path::new("resources/btn_hover.png"),
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
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => {
                self.state_result = StateResult::Pop;
            }
            _ => (),
        }

        for (_k, button) in self.buttons.iter_mut() {
            button.process_event(event);
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
