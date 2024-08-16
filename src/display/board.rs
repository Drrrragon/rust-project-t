use std::{io::stdout, sync::LazyLock};
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor::MoveTo,
};
use crate::display::cursor::BaseCursor;

use super::common::charset::{
    BaseCharset,
    get_base,
};

static BASE: LazyLock<BaseCharset> = std::sync::LazyLock::new(get_base);

pub(crate) fn clear_board() {
    // 화면을 지우고 커서를 홈 위치로 이동
    execute!(
        stdout(),
        Clear(ClearType::All),   // 화면 전체를 지움
        MoveTo(0, 0)             // 커서를 홈 위치로 이동
    ).unwrap();
}

pub(crate) fn init_board(layer: &mut Vec<Vec<char>>) {
    let block_char: char = BASE.get_block();
    let empty_char: char = BASE.get_empty();
    let layer_outer_length = layer.len(); // 외부 벡터의 길이
    let layer_inner_length = layer.get(1).unwrap().len();
    for (i, row) in layer.iter_mut().enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            if i == 0 || i == layer_outer_length - 1 {
                *col = block_char;
            }
            else {
                if j == 0 || j == layer_inner_length -1 {
                    *col = block_char;
                }
                else {
                    *col = empty_char;
                }
            }
        }
    }
}

pub(crate) fn init_title_board(layer: &mut Vec<Vec<char>>, player_cursor: &mut BaseCursor, start_cursor: &mut BaseCursor) {
    let cursor_char: char = BASE.get_cursor();
    for (i, row) in layer.iter_mut().enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            match i {
                3 => {
                    match j {
                        10 => *col = 't',
                        11 => *col = 'e',
                        12 => *col = 's',
                        13 => *col = 't',
                        17 => *col = 't',
                        18 => *col = 'i',
                        19 => *col = 't',
                        20 => *col = 'l',
                        21 => *col = 'e',
                        _ => {},
                    }
                },
                6 => {
                    match j {
                        7 => {
                            player_cursor.x = i as u16;
                            player_cursor.y = j as u16;
                            start_cursor.x = i as u16;
                            start_cursor.y = j as u16;
                            *col = cursor_char;
                        },
                        10 => *col = 's',
                        11 => *col = 't',
                        12 => *col = 'a',
                        13 => *col = 'r',
                        14 => *col = 't',
                        17 => *col = 'g',
                        18 => *col = 'a',
                        19 => *col = 'm',
                        20 => *col = 'e',
                        _ => {},
                    }
                },
                7 => {
                    match j {
                        10 => *col = 'e',
                        11 => *col = 'x',
                        12 => *col = 'i',
                        13 => *col = 't',
                        17 => *col = 'g',
                        18 => *col = 'a',
                        19 => *col = 'm',
                        20 => *col = 'e',
                        _ => {},
                    }
                },
                _ => {}, // 기본 케이스 (디폴트)
            }
        }
    }
}

pub(crate) fn start_title_board(layer: &mut Vec<Vec<char>>) {
    for (i, row) in layer.iter_mut().enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            match i {
                5 => {
                    match j {
                        10 => *col = 'S',
                        12 => *col = 'T',
                        14 => *col = 'A',
                        16 => *col = 'R',
                        18 => *col = 'T',
                        20 => *col = '!',
                        _ => {},
                    }
                },
                _ => {}, // 기본 케이스 (디폴트)
            }
        }
    }
}

pub(crate) fn end_title_board(layer: &mut Vec<Vec<char>>) {
    for (i, row) in layer.iter_mut().enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            match i {
                5 => {
                    match j {
                        8 => *col = '😭',
                        10 => *col = 'B',
                        12 => *col = 'Y',
                        14 => *col = 'E',
                        16 => *col = 'B',
                        18 => *col = 'Y',
                        20 => *col = 'E',
                        22 => *col = '😭',
                        30 => *col = BASE.get_block(),
                        32 => *col = BASE.get_empty(),
                        _ => {},
                    }
                },
                _ => {}, // 기본 케이스 (디폴트)
            }
        }
    }
}
