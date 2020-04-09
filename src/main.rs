use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::TextureCreator;
use sdl2::render::{Canvas, Texture, WindowCanvas};
use sdl2::video::WindowContext;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::Duration;

use std::cell::RefCell;
use std::rc::Rc;

use num_traits::cast::{FromPrimitive, ToPrimitive};
use num_traits::int::PrimInt;

const DIRECTION: &str = "left";

// 이건 실제 노출되는 물리적인 화면의 크기이다.
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

// 논리적인 게임데이터는 VIRTUAL_* 환경에서 돌아가는 것으로 가정한다.
const VIRTUAL_WIDTH: u32 = 400;
const VIRTUAL_HEIGHT: u32 = 300;

const WIDTH_RATIO: f32 = SCREEN_WIDTH as f32 / VIRTUAL_WIDTH as f32;
const HEIGHT_RATIO: f32 = SCREEN_HEIGHT as f32 / VIRTUAL_HEIGHT as f32;

const REVERSE_WIDTH_RATIO: f32 = 1.0 / WIDTH_RATIO;
const REVERSE_HEIGHT_RATIO: f32 = 1.0 / HEIGHT_RATIO;

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

/// 화면 입력을 컨트롤할 수 있는
/// GUI객체
pub struct GuiElement<'a> {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    texture_normal: Texture<'a>,
    texture_hover: Texture<'a>,
    is_hover: bool,
}

impl<'a> GuiElement<'a> {
    fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
        normal_path: &Path,
        hover_path: &Path,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
    ) -> GuiElement<'a> {
        let texture_normal = texture_creator.load_texture(normal_path).unwrap();
        let texture_hover = texture_creator.load_texture(hover_path).unwrap();

        GuiElement {
            x,
            y,
            w,
            h,
            texture_normal,
            texture_hover,
            is_hover: false,
        }
    }

    fn update(&mut self) {}

    fn process_event(&mut self, _event: &Event) {}

    /// 마우스 입력부분만 여기서 처리
    fn process_mouse(
        &mut self,
        x: i32,
        y: i32,
        new_buttons: &HashSet<sdl2::mouse::MouseButton>,
        old_buttons: &HashSet<sdl2::mouse::MouseButton>,
    ) {
        // x와 y는 physical 데이터
        // self.x와 self.y 는 transform 된 데이터
        let rx = transform_value(x, REVERSE_WIDTH_RATIO);
        let ry = transform_value(y, REVERSE_HEIGHT_RATIO);

        self.is_hover = rx >= self.x
            && rx <= self.x + self.w as i32
            && ry >= self.y
            && ry <= self.y + self.h as i32;

        // 버튼 press 체크
        if self.is_hover && new_buttons.contains(&sdl2::mouse::MouseButton::Left) {
            println!("Left Button is down");
        }

        // 버튼 release 체크
        if self.is_hover && old_buttons.contains(&sdl2::mouse::MouseButton::Left) {
            println!("Left Button is up");
        }
    }

    fn render(&self, canvas: &mut WindowCanvas) {
        canvas
            .copy_ex(
                if self.is_hover {
                    &self.texture_hover
                } else {
                    &self.texture_normal
                },
                None,
                Some(transform_rect(
                    &Rect::new(self.x, self.y, self.w, self.h),
                    WIDTH_RATIO,
                    HEIGHT_RATIO,
                )),
                0.,
                Some(Point::new(0, 0)),
                false,
                false,
            )
            .unwrap();
    }
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

        // 캐릭터의 w, h는 VIRTUAL_WIDTH, VIRTUAL_HEIGHT 크기의 화면에 출력된다고 가정
        // 해당하는 w, h를 SCREEN_WIDTH, SCREEN_HEIGHT에 맞추어 출력해야한다.
        // w => w * SCREEN_WIDTH / VIRTUAL_WIDTH
        // h => h * SCREEN_HEIGHT / VIRTUAL_HEIGHT

        let transformed_rect = Rect::new(
            transform_value(self.x, WIDTH_RATIO),
            transform_value(self.y, HEIGHT_RATIO),
            transform_value(self.w, WIDTH_RATIO),
            transform_value(self.h, HEIGHT_RATIO),
        );
        canvas
            .copy_ex(
                texture,
                Some(src),
                Some(transformed_rect),
                0.,
                None,
                false,
                false,
            )
            .unwrap();
    }
}

fn transform_value<T>(src: T, ratio: f32) -> T
where
    T: PrimInt + ToPrimitive + FromPrimitive,
{
    let f_value = src.to_f32().unwrap();
    FromPrimitive::from_f32(f_value * ratio).unwrap()
}

fn transform_rect(src: &Rect, ratio_w: f32, ratio_h: f32) -> Rect {
    Rect::new(
        transform_value(src.x(), ratio_w),
        transform_value(src.y(), ratio_h),
        transform_value(src.width(), ratio_w),
        transform_value(src.height(), ratio_h),
    )
}

trait States {
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
}

struct InitState<'a> {
    //font: sdl2::ttf::Font<'a, 'b>,
    //surface: sdl2::surface::Surface<'a>,
    texture: Option<Texture<'a>>,
    buttons: HashMap<String, Rc<RefCell<GuiElement<'a>>>>,
}

impl<'a> InitState<'a> {
    fn new() -> InitState<'a> {
        InitState {
            texture: None,
            buttons: HashMap::new(),
        }
    }

    fn init(
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
        let start_button = GuiElement::new(
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
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => StateResult::Push(StateInfo::Game("game")),
            _ => {
                for (_k, v) in self.buttons.iter_mut() {
                    let mut button = v.borrow_mut();
                    button.process_event(&event);
                }

                StateResult::Default
            }
        }
    }

    fn update(&mut self) -> StateResult {
        // 화면의 모든 버튼에 대한 update
        for (_k, v) in self.buttons.iter_mut() {
            let mut button = v.borrow_mut();
            button.update();
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
        let v_x = transform_value(x, REVERSE_WIDTH_RATIO);
        let v_y = transform_value(y, REVERSE_WIDTH_RATIO);
        if !new_buttons.is_empty() || !old_buttons.is_empty() {
            println!(
                "X = {:?}, Y = {:?} : {:?} -> {:?}",
                v_x, v_y, new_buttons, old_buttons
            );
        }
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
        let mut uc_vec = vec![];

        for i in 0..max_frame {
            uc_vec.push(Rect::new(i as i32 * w as i32, 0, w, h));
        }
        uc.set_animation(uc_vec);
        self.unit_char
            .insert(name.to_string(), Rc::new(RefCell::new(uc)));
    }

    fn init(
        &mut self,
        texture_creator: &'a TextureCreator<WindowContext>,
        _font_context: &'a sdl2::ttf::Sdl2TtfContext,
    ) {
        self.add_sprite(
            texture_creator,
            "image".to_string(),
            "resources/GodotPlayer.png".to_string(),
        );

        self.add_unit_char(DIRECTION, 16, 16, 4);
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
    let mut init_state = InitState::new();
    init_state.init(&texture_creator, &font_context);
    states.push(Box::new(init_state));

    let mut prev_buttons = HashSet::new();
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
                        // state 생성도 여기에서 함
                        // 각 state에서는 생성할 state를 돌려줄 수 있음
                        // 전역 state 보관함에서 넣었다 뺐다 해야함
                        match state.process_event(&event) {
                            StateResult::Push(s) => match s {
                                StateInfo::Game(_name) => {
                                    let mut game_state = GameState::new();
                                    game_state.init(&texture_creator, &font_context);
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

        // mouse 처리는 events를 가지고 함
        let mouse_state = event_pump.mouse_state();

        // Create a set of pressed Keys.
        let buttons = mouse_state.pressed_mouse_buttons().collect();

        // Get the difference between the new and old sets.
        let new_buttons = &buttons - &prev_buttons;
        let old_buttons = &prev_buttons - &buttons;

        canvas.clear();
        if let Some(state) = states.last_mut() {
            state.process_mouse(mouse_state.x(), mouse_state.y(), &new_buttons, &old_buttons);
            state.update();
            state.render(&mut canvas);
        }

        prev_buttons = buttons;
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
