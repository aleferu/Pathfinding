//extern crate core;

use std::collections::HashMap;
use std::time;
use macroquad::prelude as mq;
use macroquad::input as input_mq;

mod settings_reader;
mod squares;

// Setting up the window
fn window_conf() -> mq::Conf {
    let settings: HashMap<String, String> = settings_reader::get_settings();
    mq::Conf {
        window_title: settings.get("window_title").unwrap().to_owned(),
        window_width: settings.get("window_width").unwrap().parse().unwrap(),
        window_height: settings.get("window_height").unwrap().parse().unwrap(),
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

// Main function
#[macroquad::main(window_conf)]
async fn main() {
    let settings: HashMap<String, String> = settings_reader::get_settings();

    let square_width: usize = settings.get("square_width").unwrap().parse::<usize>().unwrap();
    let top_offset: usize = settings.get("top_offset").unwrap().parse::<usize>().unwrap();

    let mut square_collection = squares::SquareCollection::new(square_width, top_offset, mq::screen_width(), mq::screen_height());

    let mut loop_start_time = time::Instant::now();
    let mut fps_counter: usize = 0usize;
    let mut frames_drawed: usize = 0usize;
    // Window loop
    loop {
        // Background
        mq::clear_background(mq::WHITE);

        // Input
        if input_mq::is_mouse_button_down(mq::MouseButton::Left) {
            if input_mq::is_key_down(mq::KeyCode::LeftShift) {
                square_collection.change_square_type(input_mq::mouse_position(), squares::SquareType::Blank);
            } else {
                square_collection.change_square_type(input_mq::mouse_position(), squares::SquareType::Wall);
            }
        } else if input_mq::is_mouse_button_down(mq::MouseButton::Right) {
            square_collection.change_square_type(input_mq::mouse_position(), squares::SquareType::Objective);
        } else if input_mq::is_mouse_button_down(mq::MouseButton::Middle) {
            square_collection.change_square_type(input_mq::mouse_position(), squares::SquareType::Start);
        } else if input_mq::is_key_pressed(mq::KeyCode::A) {
            square_collection.a_star();
        }

        // Draw
        square_collection.draw_squares();
        draw_grid(square_width, top_offset);

        // FPS counter
        let time_elapsed_since_start = loop_start_time.elapsed().as_micros();
        if time_elapsed_since_start >= 1_000_000 {
            loop_start_time = time::Instant::now();
            frames_drawed = fps_counter;
            fps_counter = 0;
        }
        fps_counter += 1;

        let text_to_draw = format!("FPS: {frames_drawed}");
        mq::draw_text(&text_to_draw, 5f32, 25f32, 30f32, mq::BLACK);


        // FPS limit so it doesn't stress your CPU out
        let fps: f32 = 30.0; // change this
        let ideal_time: f32 = 1.0 / fps * 1_000_000.0 * (fps_counter as f32);
        let time_difference: i128 = ideal_time as i128 - time_elapsed_since_start as i128;
        if time_difference > 0 {
            std::thread::sleep(time::Duration::from_micros(time_difference as u64));
        }

        println!("{:?}\t{:?}", square_collection.is_start_square_set(), square_collection.is_objective_square_set());

        // Next frame
        mq::next_frame().await;
    }
}

// Draw grid
fn draw_grid(square_width: usize, top_offset: usize) {
    let mut x = 0usize;
    let mut y = top_offset;
    let thickness = 2f32;
    let line_color = mq::BLACK;
    let screen_width = mq::screen_width() as usize;
    let screen_height = mq::screen_height() as usize;
    while x <= screen_width {
        mq::draw_line(x as f32, top_offset as f32, x as f32, mq::screen_height(), thickness, line_color);
        x += square_width;
    }
    while y <= (screen_height as usize) {
        mq::draw_line(0f32, y as f32, mq::screen_width(), y as f32, thickness, line_color);
        y += square_width;
    }
}