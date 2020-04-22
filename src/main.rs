use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::TimerSubsystem;
use sdl_isometric::constant::*;
use sdl_isometric::states::*;
use sdl_isometric::*;
use std::collections::HashSet;
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().expect("ERROR on SDL CONTEXT");

    // Video
    let video_subsystem = sdl_context.video().expect("ERROR on Video_subsystem");
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let font_context = sdl2::ttf::init().unwrap();

    // Timer
    let mut timer_subsystem: TimerSubsystem = sdl_context.timer().unwrap();
    // Audio
    let _audio = sdl_context.audio().expect("ERROR on audio_subsystem");

    let frequency = 44_100;
    let format = sdl2::mixer::DEFAULT_FORMAT;
    let channels = sdl2::mixer::DEFAULT_CHANNELS;
    let chunk_size = 2_024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size)?;

    let _mixer_context = sdl2::mixer::init(
        sdl2::mixer::InitFlag::MP3
            | sdl2::mixer::InitFlag::FLAC
            | sdl2::mixer::InitFlag::MOD
            | sdl2::mixer::InitFlag::OGG,
    )
    .expect("ERROR ON makeing mixer context");

    let window = video_subsystem
        .window("isometric rust-sdl2 demo", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .expect("ERROR on window");

    // Renderer 만들기
    let mut canvas = window.into_canvas().build().expect("ERROR on canvas");
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().expect("ERROR on event_pump");

    // State 생성하기
    let mut states: Vec<Box<dyn States>> = vec![];
    let mut init_state = InitState::new();
    init_state.init(&texture_creator, &font_context);
    states.push(Box::new(init_state));

    let mut prev_buttons = HashSet::new();

    // delta time
    let mut dt: f64;
    let mut now: u32 = timer_subsystem.ticks();
    let mut last_time: u32 = 0;
    'running: loop {
        dt = (now - last_time) as f64 / 1000.; // 1000분의 1초로 dt를 계산한다.
        last_time = now;
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
                        state.process_event(&event, dt);
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
            state.process_mouse(
                mouse_state.x(),
                mouse_state.y(),
                &new_buttons,
                &old_buttons,
                dt,
            );
            state.update(dt);
            state.render(&mut canvas);
        }

        prev_buttons = buttons;
        canvas.present();

        // State의 최종 결과에 대한 처리

        let state_result = states.last_mut().unwrap().next_result();
        match state_result {
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
        //::std::thread::sleep(Duration::new(0, 1000));
        now = timer_subsystem.ticks();
    }

    Ok(())
}
