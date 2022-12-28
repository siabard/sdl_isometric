use std::collections::HashMap;

use hangul_jaso::Languages;
use image::io::Reader as ImageReader;
use jaso_sdl2::Fonts;

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let video = sdl_context.video().unwrap();
    let window = video
        .window("Screen Test", 320, 240)
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut screen = sdl_isometric::ascii::screen::Screen::new(320, 240, 8, 8);

    // 폰트 읽어들이기
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

    let mut fonts: HashMap<Languages, Fonts> = HashMap::new();
    fonts.insert(Languages::Ascii, Fonts::Ascii(eng_font));
    fonts.insert(Languages::Hangul, Fonts::Korean(han_font));
    'game_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    break 'game_loop;
                }
                _ => {}
            }
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));

        canvas.clear();

        screen.put_char(5, 5, 'A', Some((255, 255, 255, 255)), Some((0, 0, 0, 255)));
        screen.put_string(
            5,
            7,
            &"안녕하세요. 1234".to_string(),
            Some((255, 255, 255, 255)),
            Some((0, 255, 0, 255)),
        );

        screen.render(&fonts, &mut canvas);
        canvas.present();

        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
