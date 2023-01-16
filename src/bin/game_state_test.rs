use std::collections::HashMap;
use std::path::Path;

use sdl_isometric::ascii::entity::Tile;
use sdl_isometric::ascii::game_state::GameState;

use sdl_isometric::physics::shadow_casting::LightMap;

fn main() {
    let mut state = GameState::new();
    let mut x = 10;
    let mut y = 10;

    state.add_entity(Some((1, 1)), Some(Tile::Wall));
    state.add_entity(Some((2, 3)), Some(Tile::Ascii('c')));
    state.add_entity(None, Some(Tile::Ascii('d')));
    state.generate_rooms();

    // 320x240 해당도의 8x16 셀기준
    let mut light_map: LightMap = LightMap::new(40, 15);

    let player = state.add_entity(Some((x, y)), Some(Tile::Player));

    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video
        .window("Game State Test", 320, 240)
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut screen = sdl_isometric::ascii::screen::Screen::new(320, 240, 8, 8);

    let eng_font = jaso_sdl2::build_ascii_fonts(&Path::new("assets/bitmap_fonts/ascii-light.png"));

    let han_font =
        jaso_sdl2::build_korean_fonts(&Path::new("assets/bitmap_fonts/hangul-dkby-dinaru-2.png"));

    let mut fonts: HashMap<hangul_jaso::Languages, jaso_sdl2::Fonts> = HashMap::new();
    fonts.insert(
        hangul_jaso::Languages::Ascii,
        jaso_sdl2::Fonts::Ascii(eng_font),
    );
    fonts.insert(
        hangul_jaso::Languages::Hangul,
        jaso_sdl2::Fonts::Korean(han_font),
    );

    'game_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    break 'game_loop;
                }
                sdl2::event::Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::Up),
                    ..
                } => {
                    y = 0.max(y - 1);
                    state.entity_coord_update(player, Some((x, y)));
                }
                sdl2::event::Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::Down),
                    ..
                } => {
                    y = 14.min(y + 1);
                    state.entity_coord_update(player, Some((x, y)));
                }
                sdl2::event::Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::Left),
                    ..
                } => {
                    x = 0.max(x - 1);
                    state.entity_coord_update(player, Some((x, y)));
                }
                sdl2::event::Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::Right),
                    ..
                } => {
                    x = 19.min(x + 1);
                    state.entity_coord_update(player, Some((x, y)));
                }
                _ => {}
            }
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
        canvas.clear();
        screen.clear();
        light_map.clear_wall();
        light_map.clear_visible();

        let entities = state.entity_coord_and_tile();

        // entities 정보를 토대로 LightMap에 wall을 넣는다.
        for (&coord, _) in entities.iter().filter(|(_, &t)| t == Tile::Wall) {
            let pos = (coord.0 as i32, coord.1 as i32);
            light_map.set_wall(pos);
        }

        light_map.calculate_pov(3, (x, y));

        for (&coord, &tile) in entities.iter() {
            let pos = (coord.0 as i32, coord.1 as i32);

            match tile {
                Tile::Wall => {
                    screen.put_char(
                        coord.0 as u32,
                        coord.1 as u32,
                        '.',
                        if light_map.is_visible(&Some(pos)) {
                            Some((90, 90, 90, 255))
                        } else {
                            Some((90, 90, 90, 127))
                        },
                        if light_map.is_visible(&Some(pos)) {
                            Some((128, 128, 255, 255))
                        } else {
                            Some((128, 128, 255, 127))
                        },
                    );
                }
                Tile::Ascii(c) => {
                    screen.put_char(
                        coord.0 as u32,
                        coord.1 as u32,
                        c,
                        if light_map.is_visible(&Some(pos)) {
                            Some((90, 90, 90, 255))
                        } else {
                            Some((90, 90, 90, 127))
                        },
                        if light_map.is_visible(&Some(pos)) {
                            Some((128, 128, 255, 255))
                        } else {
                            Some((128, 128, 255, 127))
                        },
                    );
                }
                Tile::Player => {
                    screen.put_char(
                        coord.0 as u32,
                        coord.1 as u32,
                        '@',
                        Some((90, 90, 90, 255)),
                        Some((128, 128, 255, 255)),
                    );
                }
                _ => {
                    screen.put_char(
                        coord.0 as u32,
                        coord.1 as u32,
                        '.',
                        if light_map.is_visible(&Some(pos)) {
                            Some((127, 127, 127, 255))
                        } else {
                            Some((127, 127, 127, 127))
                        },
                        if light_map.is_visible(&Some(pos)) {
                            Some((0, 0, 0, 255))
                        } else {
                            Some((0, 0, 0, 127))
                        },
                    );
                }
            }
        }

        screen.render(&fonts, &mut canvas);
        canvas.present();

        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
