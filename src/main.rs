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
    for setting in settings.keys() {
        println!("{}: {}", setting, settings.get(setting).unwrap());
    };
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

    // Window loop
    loop {
        // Time start ticking
        let time_start = time::Instant::now();

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

        // Fps limit so it doesn't stress your CPU out
        // let time_elapsed: u128 = time_start.elapsed().as_nanos();
        // let maximum_frame_time: f32 = 1.0 / 120.0 * 1_000_000_000.0; // 60 frames per second as nanos = 120 ¿?¿?¿?
        // let maximum_frame_time: u128 = maximum_frame_time as u128;
        // println!("\nTime elapsed: {}, maximum {}", time_elapsed, maximum_frame_time);
        // println!("FPS: {}", mq::get_fps());
        // if time_elapsed < maximum_frame_time {
        //     let time_sleeping = (maximum_frame_time - time_elapsed) as u64 / 1_000_000;
        //     println!("Sleeping {} ms", time_sleeping);
        //     std::thread::sleep(time::Duration::from_millis(time_sleeping));
        // }

        let frame_time = mq::get_frame_time() / 1000.0;
        if frame_time > 1_000.0 / 60_000.0 {
            std::thread::sleep(time::Duration::from_millis((1000.0 / 120.0) as u64));
        }
        println!("{}", );

        // Next frame
        mq::next_frame().await
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
        y += square_width
    }
}