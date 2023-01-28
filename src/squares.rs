pub struct Square {
    visited: bool,
    x_grid: usize,
    y_grid: usize,
    x_draw: f32,
    y_draw: f32
}

impl Square {
    pub fn new(x: usize, y: usize, top_offset: f32) -> Square {

        todo!();
        /*Square {
            visited: false,
            x_grid: x,
            y_grid: y,
        }*/
    }

    pub fn is_visited(&self) -> bool {
        self.visited
    }

    pub fn got_visited(&mut self) {
        self.visited = true
    }
}

pub struct square_collection {

}