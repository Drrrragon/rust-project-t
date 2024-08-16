use std::sync::LazyLock;

use crate::display::common::charset::get_base;

use super::common::charset::BaseCharset;

static BASE: LazyLock<BaseCharset> = std::sync::LazyLock::new(get_base);
pub(crate) struct BaseCursor {
    pub x: u16,
    pub y: u16,
}

impl BaseCursor {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    // 빌더 패턴을 통한 x 설정
    pub fn with_x(mut self, x: u16) -> Self {
        self.x = x;
        self
    }

    // 빌더 패턴을 통한 y 설정
    pub fn with_y(mut self, y: u16) -> Self {
        self.y = y;
        self
    }
}

pub(crate) struct TitleCursor {
    pub player_cursor: BaseCursor,
    pub start_cursor: BaseCursor,
}
pub trait TitleCursorTrait {
    fn title_upper_cursor(&mut self, layer: &mut Vec<Vec<char>>);
    fn title_lower_cursor(&mut self, layer: &mut Vec<Vec<char>>);
    fn title_check_start_cursor(&mut self) -> bool;
}

impl TitleCursorTrait for TitleCursor {
    fn title_upper_cursor(&mut self, layer: &mut Vec<Vec<char>>) {
        // currnt cursor ( 7, 7 )
        // layer[6][7] = '↪';
        // layer[7][7] = ' ';
        let new_x = (self.player_cursor.x - 1) as usize;
        if new_x > 7 || new_x < 6 {
            return;
        }
        let current_x = self.player_cursor.x as usize;
        let current_y = self.player_cursor.y as usize;
        layer[new_x][current_y] = BASE.get_cursor();
        layer[current_x][current_y] = BASE.get_empty();
        self.player_cursor.x = new_x as u16;
    }
    fn title_lower_cursor(&mut self, layer: &mut Vec<Vec<char>>) {
        // currnt cursor ( 6, 7 )
        // layer[6][7] = ' ';
        // layer[7][7] = '↪';
        let new_x = (self.player_cursor.x + 1) as usize;
        if new_x > 7 || new_x < 6 {
            return;
        }
        let current_x = self.player_cursor.x as usize;
        let current_y = self.player_cursor.y as usize;
        layer[new_x][current_y] = BASE.get_cursor();
        layer[current_x][current_y] = BASE.get_empty();
        self.player_cursor.x = new_x as u16;
    }
    fn title_check_start_cursor(&mut self) -> bool {
        if self.player_cursor.x == self.start_cursor.x && self.player_cursor.y == self.start_cursor.y {
            return true
        }
        return false
    }
}

pub(crate) struct BlockCursor {
    pub current_cursor: BaseCursor,
    pub start_cursor: BaseCursor,
}