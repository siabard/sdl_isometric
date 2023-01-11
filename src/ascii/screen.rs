//! 콘솔 스크린 화면
//! 모든 출력은 스크린을 통해서 이루어진다.
//! x, y 좌표로 출력되며 각 좌표는 8x8 픽셀에 대응된다.
//! 그러므로 영문 반각 문자는 1x2 크기(8x16)가 되며
//! 한글 전각 문자는 2x2 크기(16x16)가 된다.
//! SDL2 렌더러를 통한 직접적인 최종 렌더링이 이루어진다.

use hangul_jaso::*;
use jaso_sdl2::*;
use sdl2::gfx::primitives::DrawRenderer;

use sdl2::render::WindowCanvas;
use std::collections::HashMap;

/// 콘솔 한 셀에 해당하는 구조체
pub struct ScreenCell {
    pub cell: char,
    pub fg: (u8, u8, u8, u8),
    pub bg: (u8, u8, u8, u8),
}

/// 콘솔을 애뮬레이션하는 객체
/// 모든 글자는 이 객체에 올린 후 한번에 렌더링한다.
pub struct Screen {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<ScreenCell>,
    pub cell_width: u32,
    pub cell_height: u32,
}

impl Screen {
    /// width : 스크린의 물리적 가로
    /// height : 스크린의 물리적 세로
    /// cell_width : 한 셀의 가로 길이
    /// cell_height : 한 셀의 세로 길이
    /// Screen 에서 표현가능한 셀의 갯수 = (width / cell_width ) * (height / cell_height)
    pub fn new(width: u32, height: u32, cell_width: u32, cell_height: u32) -> Screen {
        let mut cells = vec![];

        let cell_rows = height / cell_height;
        let cell_cols = width / cell_width;

        for _ in 0..cell_rows {
            for _ in 0..cell_cols {
                cells.push(ScreenCell {
                    cell: '\0',
                    fg: (255, 255, 255, 255),
                    bg: (0, 0, 0, 255),
                });
            }
        }

        Screen {
            width,
            height,
            cell_width,
            cell_height,
            cells,
        }
    }

    /// 단일한 문자를 스크린 객체에 올린다.
    pub fn put_char(
        &mut self,
        x: u32,
        y: u32,
        c: char,
        fb: Option<(u8, u8, u8, u8)>,
        bg: Option<(u8, u8, u8, u8)>,
    ) {
        let idx = (y * (self.width / self.cell_width) + x) as usize;

        let cell = &self.cells[idx];
        self.cells[idx] = ScreenCell {
            cell: c,
            fg: if let Some(c) = fb { c } else { cell.fg },
            bg: if let Some(c) = bg { c } else { cell.bg },
        };
    }

    /// 문장을 스크린 객체에 올린다.
    pub fn put_string(
        &mut self,
        x: u32,
        y: u32,
        s: &dyn ToString,
        fg: Option<(u8, u8, u8, u8)>,
        bg: Option<(u8, u8, u8, u8)>,
    ) {
        let default_idx = (y * (self.width / self.cell_width) + x) as usize;
        let default_cell = &self.cells[default_idx];
        let default_fg = if let Some(c) = fg { c } else { default_cell.fg };
        let default_bg = if let Some(c) = bg { c } else { default_cell.bg };

        let mut x_ = x;

        for c in s.to_string().chars() {
            let ucs_2_code = utf8_to_ucs2(&c).unwrap();
            let lang = ucs2_language(ucs_2_code);

            let idx = (y * (self.width / self.cell_width) + x_) as usize;

            self.cells[idx] = ScreenCell {
                cell: c,
                fg: default_fg,
                bg: default_bg,
            };

            // 16픽셀은 Screen에서는 2칸이다. Screen의 모든 칸은
            // 8x8 기준이다.
            x_ = x_ + if lang == Languages::Ascii { 1 } else { 2 };
        }
    }

    /// 스크린 깨끗이 지우기
    pub fn clear(&mut self) {
        for c in self.cells.iter_mut() {
            c.cell = '#';
            c.fg = (0, 0, 0, 0);
            c.bg = (0, 0, 0, 0);
        }
    }

    /// 렌더링
    pub fn render(
        &self,
        font_map: &HashMap<Languages, jaso_sdl2::Fonts>,
        canvas: &mut WindowCanvas,
    ) {
        let texture_creator = canvas.texture_creator();

        let mut texture = texture_creator
            .create_texture_target(
                sdl2::pixels::PixelFormatEnum::BGRA8888,
                self.width,
                self.height,
            )
            .unwrap();

        canvas
            .with_texture_canvas(&mut texture, |texture_canvas| {
                // 배경은 투명색으로 색칠
                texture_canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
                texture_canvas.set_draw_color(sdl2::pixels::Color::from((0, 0, 0, 0)));
                texture_canvas.clear();

                let cell_rows = self.height / self.cell_height;
                let cell_cols = self.width / self.cell_width;

                // 전체 셀에 대해 반복
                // TODO 이 부분은 memoization 등의 최적화를 통해
                // 속도를 늘릴 수 있을 것으로 기대함.
                for y in 0..cell_rows {
                    for x in 0..cell_cols {
                        let idx: usize = (y * cell_cols + x) as usize;
                        let cell: &ScreenCell = &self.cells[idx];

                        if cell.cell != '\0' {
                            let language = hangul_jaso::ucs2_language(
                                hangul_jaso::utf8_to_ucs2(&cell.cell).unwrap(),
                            );

                            match language {
                                Languages::Ascii => {
                                    if let Some(jaso_sdl2::Fonts::Ascii(font)) =
                                        font_map.get(&language)
                                    {
                                        for j in 0..16_i16 {
                                            let row = font.fonts[cell.cell as usize][j as usize];
                                            for i in 0..8_i16 {
                                                let v = (row << i) & 0x80;
                                                if v > 0 {
                                                    texture_canvas
                                                        .pixel(
                                                            i + (x * 8) as i16,
                                                            j + (y * 16) as i16,
                                                            cell.fg,
                                                        )
                                                        .unwrap();
                                                } else {
                                                    texture_canvas
                                                        .pixel(
                                                            i + (x * 8) as i16,
                                                            j + (y * 16) as i16,
                                                            cell.bg,
                                                        )
                                                        .unwrap();
                                                }
                                            }
                                        }
                                    }
                                }
                                Languages::Hangul => {
                                    if let Some(jaso_sdl2::Fonts::Korean(font)) =
                                        font_map.get(&language)
                                    {
                                        let (jaso, bul) = hangul_jaso::build_jaso_bul(&cell.cell);

                                        let cho_hex =
                                            &font.cho[(jaso.cho + bul.cho.unwrap() * 19) as usize];
                                        let mid_hex =
                                            &font.mid[(jaso.mid + bul.mid.unwrap() * 21) as usize];
                                        let jong_hex = match bul.jong {
                                            Some(jong) => {
                                                &font.jong[(jaso.jong + jong * 28) as usize]
                                            }
                                            _ => &font.jong[0],
                                        };

                                        for j in 0..16_i16 {
                                            let cho = cho_hex[j as usize];
                                            let mid = mid_hex[j as usize];
                                            let jong = jong_hex[j as usize];
                                            for i in 0..16_i16 {
                                                let vc = (cho << i) & 0x8000;
                                                let vm = (mid << i) & 0x8000;
                                                let vj = (jong << i) & 0x8000;

                                                if vc > 0 {
                                                    texture_canvas
                                                        .pixel(
                                                            i + (x * 8) as i16,
                                                            j + (y * 16) as i16,
                                                            cell.fg,
                                                        )
                                                        .unwrap();
                                                }
                                                if vm > 0 {
                                                    texture_canvas
                                                        .pixel(
                                                            i + (x * 8) as i16,
                                                            j + (y * 16) as i16,
                                                            cell.fg,
                                                        )
                                                        .unwrap();
                                                }
                                                if vj > 0 {
                                                    texture_canvas
                                                        .pixel(
                                                            i + (x * 8) as i16,
                                                            j + (y * 16) as i16,
                                                            cell.fg,
                                                        )
                                                        .unwrap();
                                                }

                                                if vc + vm + vj == 0 {
                                                    texture_canvas
                                                        .pixel(
                                                            i + (x * 8) as i16,
                                                            j + (y * 16) as i16,
                                                            cell.bg,
                                                        )
                                                        .unwrap();
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            })
            .unwrap();

        canvas.copy(
            &texture,
            sdl2::rect::Rect::new(0, 0, self.width, self.height),
            sdl2::rect::Rect::new(0, 0, self.width, self.height),
        );
    }
}
