use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::TextureCreator;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::video::WindowContext;
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

use std::cell::RefCell;
use std::rc::Rc;

const DIRECTION: &str = "left";

#[derive(PartialEq, Copy, Clone)]
enum StateInfo {
    Init(&'static str),
    Game(&'static str),
}

#[derive(PartialEq, Copy, Clone)]
enum StateResult {
    Push(StateInfo),
    Trans(StateInfo),
    Pop,
    Default,
}

/// Animation 을 수행하는 내역
/// 개별 캐릭터는 하나의 UnitCharacter 이다.
#[derive(Clone, PartialEq)]
pub struct UnitCharacter {
    hitbox: Option<Rect>,
    animation: Vec<Rect>,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    frame: u32,
    max_frame: u32,
}

impl UnitCharacter {
    /// 개별 캐릭터를 등록한다.
    pub fn new(w: u32, h: u32, max_frame: u32) -> UnitCharacter {
        UnitCharacter {
            hitbox: None,
            animation: vec![],
            x: 0,
            y: 0,
            w: w,
            h: h,
            frame: 0,
            max_frame: max_frame,
        }
    }

    /// hitbox를 등록한다.
    pub fn set_hitbox(&mut self, x: i32, y: i32, w: u32, h: u32) {
        self.hitbox = Some(Rect::new(x, y, w, h));
    }

    /// animation을 등록한다.
    pub fn set_animation(&mut self, frames: Vec<Rect>) {
        self.animation = frames;
    }

    /// 해당 캐릭터를 움직이게한다.
    pub fn update(&mut self) {
        // 뭔가를 합니다.
        self.frame = self.frame + 1;
        if self.frame >= self.max_frame {
            self.frame = 0;
        }
    }

    /// 해당 캐릭터를 canvas에 노출합니다.
    pub fn render(&self, canvas: &mut WindowCanvas, texture: &Texture) {
        let src: Rect = self.animation[self.frame as usize];

        canvas
            .copy_ex(
                texture,
                Some(src),
                Some(Rect::new(self.x, self.y, self.w, self.h)),
                0.,
                None,
                false,
                false,
            )
            .unwrap();
    }
}

trait States {
    fn process_event(&mut self, event: &sdl2::event::Event) -> StateResult;
    fn update(&mut self) -> StateResult;
    fn render(&self, canvas: &mut WindowCanvas) -> StateResult;
}

struct InitState<'a> {
    //font: sdl2::ttf::Font<'a, 'b>,
    //surface: sdl2::surface::Surface<'a>,
    texture: Texture<'a>,
}

impl<'a> InitState<'a> {
    fn new(
        font_context: &'a sdl2::ttf::Sdl2TtfContext,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> InitState<'a> {
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
        InitState { texture }
    }
}

impl<'a> States for InitState<'a> {
    fn process_event(&mut self, event: &sdl2::event::Event) -> StateResult {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => StateResult::Push(StateInfo::Game("game")),
            _ => StateResult::Default,
        }
    }

    fn update(&mut self) -> StateResult {
        StateResult::Default
    }

    fn render(&self, canvas: &mut WindowCanvas) -> StateResult {
        // 화면에 텍스트를 출력하기

        canvas
            .copy_ex(
                &self.texture,
                None,
                Some(Rect::new(0, 0, 400, 300)),
                0.,
                Some(Point::new(0, 0)),
                false,
                false,
            )
            .unwrap();

        StateResult::Default
    }
}

struct GameState<'a> {
    sprites: HashMap<String, Rc<RefCell<Texture<'a>>>>,
    unit_char: HashMap<String, Rc<RefCell<UnitCharacter>>>,
}

impl<'a> GameState<'a> {
    fn new() -> GameState<'a> {
        let sprites = HashMap::new();

        GameState {
            sprites,
            unit_char: HashMap::new(),
        }
    }

    fn add_sprite(
        &mut self,
        creator: &'a TextureCreator<WindowContext>,
        key: String,
        path: String,
    ) {
        let texture = creator.load_texture(Path::new(&path)).unwrap();
        self.sprites.insert(key, Rc::new(RefCell::new(texture)));
    }

    fn add_unit_char(&mut self, name: &'static str, w: u32, h: u32, max_frame: u32) {
        let mut uc: UnitCharacter = UnitCharacter::new(w, h, max_frame);
        uc.set_animation(vec![Rect::new(0, 0, w, h), Rect::new(w as i32, 0, w, h)]);
        self.unit_char
            .insert(name.to_string(), Rc::new(RefCell::new(uc)));
    }
}

impl<'a> States for GameState<'a> {
    fn process_event(&mut self, event: &sdl2::event::Event) -> StateResult {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => StateResult::Pop,
            _ => StateResult::Default,
        }
    }

    fn update(&mut self) -> StateResult {
        let unit_char_refcell = self.unit_char.get(&DIRECTION.to_string()).unwrap();
        let mut unit_char = unit_char_refcell.borrow_mut();

        unit_char.update();

        StateResult::Default
    }
    fn render(&self, canvas: &mut WindowCanvas) -> StateResult {
        // 모든 스프라이트를 WindowCanvas 에 출력..
        // 다 좋은데... x,y 좌표는??
        let texture_refcell = self.sprites.get(&"image".to_string()).unwrap();
        let texture = texture_refcell.borrow();

        let unit_char_refcell = self.unit_char.get(&DIRECTION.to_string()).unwrap();
        let unit_char = unit_char_refcell.borrow();

        unit_char.render(canvas, &texture);
        StateResult::Default
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().expect("ERROR on SDL CONTEXT");
    let video_subsystem = sdl_context.video().expect("ERROR on Video_subsystem");
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let font_context = sdl2::ttf::init().unwrap();
    let window = video_subsystem
        .window("isometric rust-sdl2 demo", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .expect("ERROR on window");

    // Renderer 만들기
    let mut canvas = window.into_canvas().build().expect("ERROR on canvas");
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().expect("ERROR on event_pump");

    let mut states: Vec<Box<dyn States>> = vec![];
    states.push(Box::new(InitState::new(&font_context, &texture_creator)));
    /*
    let mut game_state = GameState::new();
    game_state.add_sprite(
        &texture_creator,
        "image".to_string(),
        "resources/image.png".to_string(),
    );
     */

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {
                    // 가장 상단의 sates에 대한 처리
                    // 이 초기화 루틴을 어덯게 빼야하지??
                    // ron 파일을 만들어서 읽어들일까?
                    // 초기화를 하는 루틴이 필요하긴한데
                    // 어떤 데이터를 초기화하는데 이용해야할까?
                    //

                    if let Some(state) = states.last_mut() {
                        match state.process_event(&event) {
                            StateResult::Push(s) => match s {
                                StateInfo::Game(_name) => {
                                    let mut game_state = GameState::new();
                                    game_state.add_sprite(
                                        &texture_creator,
                                        "image".to_string(),
                                        "resources/GodotPlayer.png".to_string(),
                                    );

                                    game_state.add_unit_char(DIRECTION, 16, 16, 2);
                                    states.push(Box::new(game_state));
                                }
                                _ => (),
                            },

                            StateResult::Pop => {
                                states.pop().unwrap();
                            }
                            _ => (),
                        }
                    }
                }
            }
        }

        // The rest of the game loop goes here...

        // state 생성도 여기에서 함
        // 각 state에서는 생성할 state를 돌려줄 수 있음
        // 전역 state 보관함에서 넣었다 뺐다 해야함

        //draw(&mut canvas, Color::RGB(i, 64, 255 - i), Some(&texture));
        canvas.clear();
        if let Some(state) = states.last_mut() {
            state.update();
            state.render(&mut canvas);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
