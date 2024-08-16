pub mod display;

use crate::display::render;
use crate::display::board::{
    init_board,
    clear_board
};
use crate::display::cursor::{
    TitleCursor,
    BaseCursor,
    TitleCursorTrait,
};
use std::process::exit;
use std::{thread, u16};
use std::io::{stdin, stdout, Write};
use display::board::{end_title_board, init_title_board, start_title_board};

use tokio::time::{sleep, Duration};
use crossterm::{
    event::{read, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode},
    terminal::{Clear, ClearType},
    cursor::MoveTo,
};

async fn title_loop(mut layer: &mut Vec<Vec<char>>) {
    let mut title_cursor = TitleCursor {
        player_cursor: BaseCursor::new(7, 7),
        start_cursor: BaseCursor::new(7, 7),
    };
    init_board(&mut layer);
    init_title_board(&mut layer, &mut title_cursor.player_cursor, &mut title_cursor.start_cursor);
    render(&layer);
    
    enable_raw_mode().expect("Failed to enable raw mode");
    
    loop {
        match read().expect("Failed to read event") {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Up => {
                        title_cursor.title_upper_cursor(&mut layer);
                    },
                    KeyCode::Down => {
                        title_cursor.title_lower_cursor(&mut layer);
                    },
                    KeyCode::Enter => {
                        if title_cursor.title_check_start_cursor() {
                            for _i in 0..=2 {
                                init_board(&mut layer);
                                render(&layer);
                                sleep(Duration::from_millis(100)).await;
                                start_title_board(&mut layer);
                                render(&layer);
                                sleep(Duration::from_millis(300)).await;
                            }
                            sleep(Duration::from_millis(1700)).await;
                        }
                        else {
                            for _i in 0..=3 {
                                init_board(&mut layer);
                                render(&layer);
                                sleep(Duration::from_millis(100)).await;
                                end_title_board(&mut layer);
                                render(&layer);
                                sleep(Duration::from_millis(150)).await;
                            }
                            sleep(Duration::from_millis(2000)).await;
                            break;
                        }
                    }
                    _ => (),
                }
            },
            _ => (),
        }
        sleep(Duration::from_millis(10)).await;
        render(&layer);
    }
    disable_raw_mode().expect("Failed to disable raw mode");
    exit(0)
}

async fn main_loop(layer: &mut Vec<Vec<char>>) {
    enable_raw_mode().expect("Failed to enable raw mode");
    loop {
        render(&layer);
        sleep(Duration::from_millis(100)).await;
    }
    disable_raw_mode().expect("Failed to disable raw mode");
}

#[tokio::main]
async fn main() {
    let mut layer: Vec<Vec<char>> = vec![vec![' '; 33]; 10];
    
    title_loop(&mut layer).await;
    main_loop(&mut layer).await;
}

