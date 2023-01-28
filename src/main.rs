use std::collections::HashMap;
use macroquad::prelude as mq;

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

    // Window loop
    loop {
        // Background
        mq::clear_background(mq::WHITE);
        draw_grid(
            settings.get("square_width").unwrap().parse::<f32>().unwrap(),
            settings.get("top_offset").unwrap().parse::<f32>().unwrap()
        );

        mq::next_frame().await
    }
}

// Draw grid
fn draw_grid(square_width: f32, top_offset: f32) {
    let mut x = 0f32;
    let mut y = top_offset;
    let thickness = 2f32;
    let line_color = mq::BLACK;
    while x <= mq::screen_width() {
        mq::draw_line(x, top_offset, x, mq::screen_height(), thickness, line_color);
        x += square_width;
    }
    while y <= mq::screen_height() {
        mq::draw_line(0f32, y, mq::screen_width(), y, thickness, line_color);
        y += square_width
    }
}