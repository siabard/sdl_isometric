//! 콘솔 스크린 화면
//! 모든 출력은 스크린을 통해서 이루어진다.
//! x, y 좌표로 출력되며 각 좌표는 8x8 픽셀에 대응된다.
//! 그러므로 영문 반각 문자는 1x2 크기(8x16)가 되며
//! 한글 전각 문자는 2x2 크기(16x16)가 된다.
//! SDL2 렌더러를 통한 직접적인 최종 렌더링이 이루어진다.

use hangul_jaso::*;
use jaso_sdl2::*;
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
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Screen {
        let mut cells = vec![];

        for _ in 0..height {
            for _ in 0..width {
                cells.push(ScreenCell {
                    cell: ' ',
                    fg: (255, 255, 255, 255),
                    bg: (0, 0, 0, 255),
                });
            }
        }

        Screen {
            width,
            height,
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
        let idx = (y * self.width + x) as usize;

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
        let default_idx = (y * self.width + x) as usize;
        let default_cell = &self.cells[default_idx];
        let default_fg = if let Some(c) = fg { c } else { default_cell.fg };
        let default_bg = if let Some(c) = bg { c } else { default_cell.bg };

        let mut x_ = x;

        for c in s.to_string().chars() {
            let ucs_2_code = utf8_to_ucs2(&c).unwrap();
            let lang = ucs2_language(ucs_2_code);

            let idx = (y * self.width + x_) as usize;

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

    /// 렌더링
    pub fn render(&self, font: &HashMap<String, Fonts>, canvas: &mut WindowCanvas) {}
}
