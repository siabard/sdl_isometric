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

enum StateResult {
    Default,
}

trait States {
    fn update(&mut self) -> StateResult;
    fn render(&self, canvas: &mut WindowCanvas) -> StateResult;
}

struct GameState<'a> {
    sprites: HashMap<String, Rc<RefCell<Texture<'a>>>>,
}

impl<'a> GameState<'a> {
    fn new() -> GameState<'a> {
        let sprites = HashMap::new();

        GameState { sprites }
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
}

impl<'a> States for GameState<'a> {
    fn update(&mut self) -> StateResult {
        StateResult::Default
    }

    fn render(&self, canvas: &mut WindowCanvas) -> StateResult {
        // 모든 스프라이트를 WindowCanvas 에 출력..
        // 다 좋은데... x,y 좌표는??
        let texture_refcell = self.sprites.get(&"image".to_string()).unwrap();
        let texture = texture_refcell.borrow();

        canvas
            .copy_ex(
                &*texture,
                None,
                Some(Rect::new(0, 0, 400, 300)),
                0.,
                Some(Point::new(0, 0)),
                false,
                false,
            )
            .unwrap();

        canvas
            .copy_ex(
                &*texture,
                None,
                Some(Rect::new(401, 301, 400, 300)),
                0.,
                Some(Point::new(0, 0)),
                false,
                false,
            )
            .unwrap();
        StateResult::Default
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().expect("ERROR on SDL CONTEXT");
    let video_subsystem = sdl_context.video().expect("ERROR on Video_subsystem");
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
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
    let mut i = 0;

    let mut game_state = GameState::new();
    game_state.add_sprite(
        &texture_creator,
        "image".to_string(),
        "resources/image.png".to_string(),
    );
    'running: loop {
        i = (i + 1) % 255;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        // state 생성도 여기에서 함
        // 각 state에서는 생성할 state를 돌려줄 수 있음
        // 전역 state 보관함에서 넣었다 뺐다 해야함

        //draw(&mut canvas, Color::RGB(i, 64, 255 - i), Some(&texture));
        canvas.clear();
        game_state.update();
        game_state.render(&mut canvas);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
