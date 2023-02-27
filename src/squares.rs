use macroquad::prelude as mq;

#[derive(Clone)]
#[derive(PartialEq)]
pub enum SquareType {
     Wall,
     Objective,
     Start,
     Blank
 }

// Each grid cell
#[derive(Clone)]
pub struct Square {
    visited: bool,
    x_grid: f32,
    y_grid: f32,
    square_type: SquareType,
}

impl Square {
    pub fn new(x: f32, y: f32) -> Square {
        Square {
            visited: false,
            x_grid: x,
            y_grid: y,
            square_type: SquareType::Blank
        }
    }

    pub fn is_visited(&self) -> bool {
        self.visited
    }

    pub fn got_visited(&mut self) {
        self.visited = true;
    }

    pub fn set_square_type(&mut self, square_type: SquareType) {
        self.square_type = square_type;
    }

    pub fn get_square_type(&self) -> &SquareType {
        &self.square_type
    }

    fn draw(&self, square_width: &f32, top_offset: &f32) {
        let x_coord = self.x_grid * square_width;
        let y_coord = self.y_grid * square_width + top_offset;
        let color = match self.square_type {
            SquareType::Blank => mq::WHITE,
            SquareType::Wall=> mq::BLACK,
            SquareType::Start=> mq::GREEN,
            SquareType::Objective => mq::RED,
        };
        mq::draw_rectangle(x_coord, y_coord, *square_width, *square_width, color);
    }
}


pub struct SquareCollection {
    square_width: f32,
    top_offset: f32,
    squares: Vec<Vec<Square>>,
    objective_square: (usize, usize),
    objective_square_set: bool,
    start_square: (usize, usize),
    start_square_set: bool
}


impl SquareCollection {

    pub fn new(square_width: &f32, top_offset: &f32, screen_width: &f32, screen_height: &f32) -> SquareCollection {
        let squares: Vec<Vec<Square>> = SquareCollection::build_squares(square_width, top_offset, screen_width, screen_height);
        SquareCollection {
            square_width: *square_width,
            top_offset: *top_offset,
            squares: squares,
            objective_square: (0, 0),
            objective_square_set: false,
            start_square: (0, 0),
            start_square_set: false
        }
    }

    fn build_squares(square_width: &f32, top_offset: &f32, screen_width: &f32, screen_height: &f32) -> Vec<Vec<Square>> {
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

    fn get_square_from_mouse(&self, mouse_pos: (f32, f32)) -> (f32, f32) {
        let mouse_x = mouse_pos.0 / self.square_width;
        let mouse_x = mouse_x.clamp(0f32, (self.squares.len() - 1) as f32);
        let mouse_y = (mouse_pos.1 - self.top_offset) / self.square_width;
        let mouse_y = mouse_y.clamp(0f32, (self.squares[0].len() - 1) as f32);
        (mouse_x, mouse_y)
    }

    pub fn change_square_type(&mut self, mouse_pos: (f32, f32), square_type: SquareType) {
        if mouse_pos.1 > self.top_offset {
            let (mouse_x, mouse_y): (f32, f32) = self.get_square_from_mouse(mouse_pos);
            let mouse_x_index = mouse_x as usize;
            let mouse_y_index = mouse_y as usize;
            let previous_type = self.squares[mouse_x_index][mouse_y_index].get_square_type().clone();
            match square_type {
                SquareType::Start => {
                    if self.start_square_set {
                        self.squares[self.start_square.0][self.start_square.1].set_square_type(SquareType::Blank);
                    }
                    self.start_square = (mouse_x_index, mouse_y_index);
                    self.start_square_set = true;
                },
                SquareType::Objective => {
                    if self.objective_square_set {
                        self.squares[self.objective_square.0][self.objective_square.1].set_square_type(SquareType::Blank);
                    }
                    self.objective_square = (mouse_x_index, mouse_y_index);
                    self.objective_square_set = true;
                },
                _ => {  }
            }
            self.squares[mouse_x_index][mouse_y_index].set_square_type(square_type.clone());
            match previous_type {
                SquareType::Start | SquareType::Objective => {
                    if square_type != previous_type {
                        match previous_type {
                            SquareType::Start => { self.start_square_set = false; }
                            SquareType::Objective => { self.objective_square_set = false; }
                            _ => {  }
                        }
                    }
                }
                _ => {  }
            }
        }
    }

    pub fn is_start_square_set(&self) -> bool {
        self.start_square_set
    }

    pub fn is_objective_square_set(&self) -> bool {
        self.objective_square_set
    }
}
