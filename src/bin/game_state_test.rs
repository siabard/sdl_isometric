use std::collections::HashMap;

use sdl_isometric::ascii::entity::Tile;
use sdl_isometric::ascii::game_state::GameState;

use image::io::Reader as ImageReader;
use sdl2::gfx::primitives::DrawRenderer;

fn main() {
    let mut state = GameState::new();
    let mut x = 10;
    let mut y = 10;

    state.add_entity(Some((1, 1)), Some(Tile::Wall));
    state.add_entity(Some((2, 3)), Some(Tile::Ascii('c')));
    state.add_entity(None, Some(Tile::Ascii('d')));
    let player = state.add_entity(Some((x, y)), Some(Tile::Player));

    let entities = state.entity_coord_and_tile();

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

    let mut eng_font = jaso_sdl2::AsciiFonts::default();
    let mut han_font = jaso_sdl2::KoreanFonts::default();

    let eng_img_font = ImageReader::open("assets/bitmap_fonts/ascii-light.png")
        .unwrap()
        .decode()
        .unwrap();

    let han_img_font = ImageReader::open("assets/bitmap_fonts/hangul-dkby-dinaru-2.png")
        .unwrap()
        .decode()
        .unwrap();

    // 영문 가로 16글자, 세로 8글자, 각 글자는 8x16
    for y in 0..8 {
        for x in 0..16 {
            let rows = jaso_sdl2::image2hex(&eng_img_font, x * 8, y * 16, 8, 16);
            eng_font.fonts.push(rows);
        }
    }

    // 한글 가로 28글자, 세로 16글자(8,4,4), 각 글자는 16x16
    // 한글 초성 8벌 : 19 : 32*19*8 = 4864
    for y in 0..8 {
        for x in 0..19 {
            let rows = jaso_sdl2::image2hex(&han_img_font, x * 16, y * 16, 16, 16);
            han_font.cho.push(rows);
        }
    }
    // 한글 중성 4벌 : 21 : 32*21*4 = 2688
    for y in 8..12 {
        for x in 0..21 {
            let rows = jaso_sdl2::image2hex(&han_img_font, x * 16, y * 16, 16, 16);
            han_font.mid.push(rows);
        }
    }
    // 한글 종성 4벌 : 28 : 32*28*4 = 3584
    for y in 12..16 {
        for x in 0..28 {
            let rows = jaso_sdl2::image2hex(&han_img_font, x * 16, y * 16, 16, 16);
            han_font.jong.push(rows);
        }
    }

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

        let entities = state.entity_coord_and_tile();
        for (&coord, &tile) in entities.iter() {
            match tile {
                Tile::Wall => {
                    screen.put_char(
                        coord.0 as u32,
                        coord.1 as u32,
                        '.',
                        Some((90, 90, 90, 255)),
                        Some((128, 128, 255, 255)),
                    );
                }
                Tile::Ascii(c) => {
                    screen.put_char(
                        coord.0 as u32,
                        coord.1 as u32,
                        c,
                        Some((90, 90, 90, 255)),
                        Some((128, 128, 255, 255)),
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
                        Some((127, 127, 127, 255)),
                        Some((0, 0, 0, 255)),
                    );
                }
            }
        }

        screen.render(&fonts, &mut canvas);
        canvas.present();

        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
