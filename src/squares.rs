// Each grid cell
pub struct Square {
    visited: bool,
    x_grid: f32,
    y_grid: f32,
}


impl Square {
    pub fn new(x: f32, y: f32) -> Square {
        Square {
            visited: false,
            x_grid: x,
            y_grid: y,
        }
    }

    pub fn is_visited(&self) -> bool {
        self.visited
    }

    pub fn got_visited(&mut self) {
        self.visited = true
    }
}


// Needed for making a vector of this things
impl Clone for Square {
    fn clone(&self) -> Self {
        Square {
            visited: false,
            x_grid: self.x_grid,
            y_grid: self.y_grid
        }
    }
}


pub struct SquareCollection {
    square_width: f32,
    top_offset: f32,
    squares: Vec<Vec<Square>>
}


impl SquareCollection {

    pub fn new(square_width: &f32, top_offset: &f32, screen_width: &f32, screen_height: &f32) -> SquareCollection {
        let squares: Vec<Vec<Square>> = SquareCollection::build_circles(square_width, top_offset, screen_width, screen_height);
        SquareCollection {
            square_width: *square_width,
            top_offset: *top_offset,
            squares: squares
        }
    }

    fn build_circles(square_width: &f32, top_offset: &f32, screen_width: &f32, screen_height: &f32) -> Vec<Vec<Square>> {
        let mut result: Vec<Vec<Square>> = Vec::new();
        let mut x_counter = 0f32;
        while x_counter * square_width < *screen_width {
            let mut y_counter = 0f32;
            let mut temp: Vec<Square> = Vec::new();
            while y_counter * square_width + top_offset < *screen_height {
                temp.push(Square::new(x_counter, y_counter));
                y_counter += 1f32;
            }
            result.push(temp.clone());
            x_counter += 1f32;
        }
        result
    }
}
