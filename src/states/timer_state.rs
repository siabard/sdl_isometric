use crate::states::*;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;

use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};

use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::video::WindowContext;

#[derive(Copy, Clone)]
struct Timer {
    pub t: f64, // elpased time
    pub b: f64, // begin value
    pub c: f64, // change over time
    pub d: f64, // duration
}

lazy_static! {
    static ref G_TIMERS: Arc<RwLock<HashMap<u32, Timer>>> = {
        let m = Arc::new(RwLock::new(HashMap::new()));

        m
    };
}

pub struct TimerState<'a> {
    texture_manager: Option<TextureManager<'a>>,
    state_result: StateResult,
}

impl<'a> TimerState<'a> {
    pub fn new() -> TimerState<'a> {
        TimerState {
            texture_manager: None,
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
    }
}

impl<'a> States for TimerState<'a> {
    fn process_event(&mut self, event: &sdl2::event::Event, _dt: f64) -> StateResult {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                self.state_result = StateResult::Pop;
            }

            Event::KeyDown {
                keycode: Some(Keycode::Num1),
                ..
            } => {
                // 신규 상태 만들기
                G_TIMERS.write().unwrap().insert(
                    1,
                    Timer {
                        t: 0.0,
                        b: 0.0,
                        c: 10.0,
                        d: 10.0,
                    },
                );
                self.state_result = StateResult::Default;
            }
            _ => self.state_result = StateResult::Default,
        }

        self.state_result
    }

    fn update(&mut self, dt: f64) -> StateResult {
        // 1번이 있으면..
        // 값을 전환하기...

        let mut g_t = G_TIMERS.write().unwrap();
        let g_v = g_t.get_mut(&1);

        match g_v {
            Some(v) => {
                let v_x = v.clone();
                if v_x.d >= v_x.t {
                    {
                        g_t.insert(
                            1,
                            Timer {
                                t: v_x.t + dt,
                                b: v_x.b,
                                c: v_x.c,
                                d: v_x.d,
                            },
                        );
                    }
                }
            }
            None => (),
        }
        StateResult::Default
    }

    fn render(&self, canvas: &mut WindowCanvas) -> StateResult {
        let g_t = G_TIMERS.read().unwrap();
        let g_v = g_t.get(&1);
        match g_v {
            Some(v) => {
                let cv = tween::in_sine(v.t, v.b, v.c, v.d);
                println!("cv => {}", cv);
                canvas.circle(50, 50, (cv * 10.0) as i16, Color::WHITE);
            }
            None => (),
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
    }

    fn next_result(&mut self) -> StateResult {
        let result = self.state_result;

        result
    }
}
