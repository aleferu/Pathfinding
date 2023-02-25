extern crate core;

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

    let square_width: f32 = settings.get("square_width").unwrap().parse::<f32>().unwrap();
    let top_offset: f32 = settings.get("top_offset").unwrap().parse::<f32>().unwrap();

    let mut square_collection = squares::SquareCollection::new(&square_width, &top_offset, &mq::screen_width(), &mq::screen_height());

    let mut loop_start_time = time::Instant::now();
    let mut fps_counter: f32 = 0.0;
    let mut frames_drawed: f32 = 0.0;
    let mut fps_vector: Vec<i32> = Vec::new();
    // Window loop
    loop {
        // Background
        mq::clear_background(mq::WHITE);

        // Input
        if input_mq::is_mouse_button_down(mq::MouseButton::Left) {
            square_collection.create_wall(input_mq::mouse_position());
        } else if input_mq::is_mouse_button_down(mq::MouseButton::Right) {
            square_collection.set_objective(input_mq::mouse_position());
        } else if input_mq::is_mouse_button_down(mq::MouseButton::Middle) {
            square_collection.set_start_square(input_mq::mouse_position());
        }

        // Draw
        square_collection.draw_squares();
        draw_grid(&square_width, &top_offset);

        // FPS counter
        let time_elapsed_since_start = loop_start_time.elapsed().as_micros();
        fps_counter += 1.0;
        if time_elapsed_since_start >= 1_000_000 {
            loop_start_time = time::Instant::now();
            frames_drawed = fps_counter;
            fps_counter = 0.0;
        }

        let text_to_draw = format!("FPS (me): {frames_drawed}\nFPS (mq): {mq_fps}", mq_fps = fps_macroquad(&mut fps_vector, mq::get_fps()));
        mq::draw_text(&text_to_draw, 5f32, 25f32, 30f32, mq::BLACK);


        // FPS limit so it doesn't stress your CPU out
        let fps: f32 = 60.0; // change this
        let ideal_time: f32 = 1.0 / fps * 1_000_000.0 * fps_counter;
        let time_difference: i128 = ideal_time as i128 - time_elapsed_since_start as i128;
        if time_difference > 0 {
            std::thread::sleep(time::Duration::from_micros(time_difference as u64));
        }

        // Next frame
        mq::next_frame().await;
    }
}

// Draw grid
fn draw_grid(square_width: &f32, top_offset: &f32) {
    let mut x = 0f32;
    let mut y = *top_offset;
    let thickness = 2f32;
    let line_color = mq::BLACK;
    while x <= mq::screen_width() {
        mq::draw_line(x, *top_offset, x, mq::screen_height(), thickness, line_color);
        x += square_width;
    }
    while y <= mq::screen_height() {
        mq::draw_line(0f32, y, mq::screen_width(), y, thickness, line_color);
        y += square_width;
    }
}

// fps computed by mq
fn fps_macroquad(fps_vector: &mut Vec<i32>, fps_now: i32) -> i32 {
    fps_vector.insert(0, fps_now);
    if fps_vector.len() == 100 {
        fps_vector.pop();
    }
    let mut sum: i32 = 0;
    let mut bigger: i32 = 0;
    for element in & *fps_vector {
        sum += *element;
        if *element > bigger {
            bigger = *element;
        }
    }
    sum -= bigger;
    ((sum as f32) / (fps_vector.len() as f32)) as i32
}