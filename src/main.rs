use macroquad::prelude as mq;

pub mod toml_reader;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        mq::clear_background(mq::RED);

        mq::draw_line(40.0, 40.0, 100.0, 200.0, 15.0, mq::BLUE);
        mq::draw_rectangle(mq::screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, mq::GREEN);
        mq::draw_circle(mq::screen_width() - 30.0, mq::screen_height() - 30.0, 15.0, mq::YELLOW);

        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        text();

        mq::next_frame();//.await
    }
}

fn text() {
    mq::draw_text("IT WORKS!", 20.0, 20.0, 30.0, mq::DARKGRAY);
}