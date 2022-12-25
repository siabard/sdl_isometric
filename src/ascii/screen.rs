//! 콘솔 스크린 화면
//! 모든 출력은 스크린을 통해서 이루어진다.
//! x, y 좌표로 출력되며 각 좌표는 8x8 픽셀에 대응된다.
//! 그러므로 영문 반각 문자는 1x2 크기(8x16)가 되며
//! 한글 전각 문자는 2x2 크기(16x16)가 된다.
//! SDL2 렌더러를 통한 직접적인 최종 렌더링이 이루어진다.

use hangul_jaso::*;

pub enum Cell {
    HalfCell(char),
    FullCell(char),
}

pub struct ScreenCell {
    pub cell: Cell,
    pub fg: (u8, u8, u8, u8),
    pub bg: (u8, u8, u8, u8),
}

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
                    cell: Cell::HalfCell('.'),
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

    pub fn putchar(
        &mut self,
        x: u32,
        y: u32,
        c: char,
        fb: Option<(u8, u8, u8, u8)>,
        bg: Option<(u8, u8, u8, u8)>,
    ) {
        let idx = (y * self.width + x) as usize;

        let code = utf8_to_ucs2(&c).unwrap();
        let lang = ucs2_language(code);

        let screen_cell = match lang {
            Languages::Ascii => Cell::HalfCell(c),
            Languages::Hangul => Cell::FullCell(c),
            Languages::Kana => Cell::FullCell(c),
            Languages::Arrow => Cell::HalfCell(c),
            Languages::HangulJamo => Cell::FullCell(c),
            _ => Cell::HalfCell(' '),
        };

        let cell = &self.cells[idx];
        self.cells[idx] = ScreenCell {
            cell: screen_cell,
            fg: if let Some(c) = fb { c } else { cell.fg },
            bg: if let Some(c) = bg { c } else { cell.bg },
        };
    }
}
