use macroquad::prelude as mq;

// Each grid cell
pub struct Square {
    visited: bool,
    x_grid: f32,
    y_grid: f32,
    drawable: bool,
    wall: bool,
    objective: bool
}


impl Square {
    pub fn new(x: f32, y: f32) -> Square {
        Square {
            visited: false,
            x_grid: x,
            y_grid: y,
            drawable: false,
            wall: false,
            objective: false
        }
    }

    pub fn is_visited(&self) -> bool {
        self.visited
    }

    pub fn got_visited(&mut self) {
        self.visited = true;
        self.update_drawable();
    }

    pub fn got_walled(&mut self) {
        if !self.objective {
            self.wall = true;
            self.update_drawable();
        }
    }

    pub fn set_objective(&mut self) {
        self.objective = true;
        self.wall = false;
        self.update_drawable();
    }

    pub fn remove_objective(&mut self) {
        self.objective = false;
    }

    fn update_drawable(&mut self) {
        self.drawable =  self.visited || self.wall || self.objective;
    }

    fn draw(&self, square_width: &f32, top_offset: &f32) {
        if self.drawable {
            let x_coord = self.x_grid * square_width;
            let y_coord = self.y_grid * square_width + top_offset;
            let mut color: mq::Color = mq::WHITE;
            if self.objective {
                color = mq::RED;
            } else if self.wall {
                color = mq::BLACK;
            } else if self.visited {
                color = mq::BLUE;
            }
            mq::draw_rectangle(x_coord, y_coord, *square_width, *square_width, color);
        }
    }
}


// Needed for making a vector of this things
impl Clone for Square {
    fn clone(&self) -> Self {
        Square::new(self.x_grid, self.y_grid)
    }
}


pub struct SquareCollection {
    square_width: f32,
    top_offset: f32,
    squares: Vec<Vec<Square>>,
    objective: (usize, usize),
    objective_set: bool
}


impl SquareCollection {

    pub fn new(square_width: &f32, top_offset: &f32, screen_width: &f32, screen_height: &f32) -> SquareCollection {
        let mut squares: Vec<Vec<Square>> = SquareCollection::build_circles(square_width, top_offset, screen_width, screen_height);
        SquareCollection {
            square_width: *square_width,
            top_offset: *top_offset,
            squares: squares,
            objective: (0, 0),
            objective_set: false
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

    pub fn draw_squares(&self) {
        for column in &self.squares {
            for square in column {
                square.draw(&self.square_width, &self.top_offset);
            }
        }
    }

    pub fn create_wall(&mut self, mouse_pos: (f32, f32)) {
        if mouse_pos.1 > self.top_offset {
            let (mouse_x, mouse_y): (f32, f32) = self.get_square_from_mouse(mouse_pos);
            self.squares[mouse_x as usize][mouse_y as usize].got_walled();
        }
    }

    fn get_square_from_mouse(&self, mouse_pos: (f32, f32)) -> (f32, f32) {
        let mouse_x = mouse_pos.0 / self.square_width;
        let mouse_x = mouse_x.clamp(0f32, (self.squares.len() - 1) as f32);
        let mouse_y = (mouse_pos.1 - self.top_offset) / self.square_width;
        let mouse_y = mouse_y.clamp(0f32, (self.squares[0].len() - 1) as f32);
        (mouse_x, mouse_y)
    }

    pub fn set_objective(&mut self, mouse_pos: (f32, f32)) {
        if mouse_pos.1 > self.top_offset {
            let (mouse_x, mouse_y): (f32, f32) = self.get_square_from_mouse(mouse_pos);
            let mouse_x_index = mouse_x as usize;
            let mouse_y_index = mouse_y as usize;
            if self.objective != (mouse_x_index, mouse_y_index) {
                if self.objective_set {
                    self.squares[self.objective.0][self.objective.1].remove_objective();
                }
                self.squares[mouse_x_index][mouse_y_index].set_objective();
                self.objective = (mouse_x_index, mouse_y_index);
                self.objective_set = true;
            }
        }
    }
}
