use macroquad::prelude as mq;
use std::collections::{HashMap, HashSet};
use rand::Rng;


#[derive(Clone)]
#[derive(PartialEq)]
pub enum SquareType {
     Wall,
     Objective,
     Start,
     Blank,
     Visited,
     Solution
 }


// Each grid cell
#[derive(Clone)]
pub struct Square {
    x_grid: usize,
    y_grid: usize,
    square_type: SquareType,
}


impl Square {
    pub fn new(x: usize, y: usize) -> Square {
        Square {
            x_grid: x,
            y_grid: y,
            square_type: SquareType::Blank
        }
    }


    pub fn set_square_type(&mut self, square_type: SquareType) {
        if !(self.square_type == SquareType::Start && square_type == SquareType::Visited) {
            self.square_type = square_type;
        }
    }


    pub fn get_square_type(&self) -> &SquareType {
        &self.square_type
    }


    fn draw(&self, square_width: usize, top_offset: usize) {
        let x_coord = self.x_grid * square_width;
        let y_coord = self.y_grid * square_width + top_offset;
        let color = match self.square_type {
            SquareType::Blank => mq::WHITE,
            SquareType::Wall=> mq::BLACK,
            SquareType::Start=> mq::GREEN,
            SquareType::Objective => mq::RED,
            SquareType::Visited => mq::BLUE,
            SquareType::Solution => mq::BROWN
        };
        mq::draw_rectangle(x_coord as f32, y_coord as f32, square_width as f32, square_width as f32, color);
    }
}


pub struct SquareCollection {
    square_width: usize,
    top_offset: usize,
    squares: Vec<Vec<Square>>,
    objective_square: (usize, usize),
    objective_square_set: bool,
    start_square: (usize, usize),
    start_square_set: bool,
    states: Vec<Vec<Vec<SquareType>>>,
    current_state: usize,
}


impl SquareCollection {
    pub fn new(square_width: usize, top_offset: usize, screen_width: f32, screen_height: f32) -> SquareCollection {
        let squares: Vec<Vec<Square>> = SquareCollection::build_squares(square_width, top_offset, screen_width, screen_height);
        SquareCollection {
            square_width: square_width,
            top_offset: top_offset,
            squares: squares,
            objective_square: (0, 0),
            objective_square_set: false,
            start_square: (0, 0),
            start_square_set: false,
            states: Vec::new(),
            current_state: 0,
        }
    }


    fn build_squares(square_width: usize, top_offset: usize, screen_width: f32, screen_height: f32) -> Vec<Vec<Square>> {
        let mut result: Vec<Vec<Square>> = Vec::new();
        let mut x_counter = 0;
        let screen_width = screen_width as usize;
        let screen_height = screen_height as usize;
        while x_counter * square_width < screen_width {
            let mut y_counter = 0;
            let mut temp: Vec<Square> = Vec::new();
            while y_counter * square_width + top_offset < screen_height {
                temp.push(Square::new(x_counter, y_counter));
                y_counter += 1;
            }
            result.push(temp.clone());
            x_counter += 1;
        }
        result
    }


    pub fn draw_squares(&self) {
        for column in &self.squares {
            for square in column {
                square.draw(self.square_width as usize, self.top_offset as usize);
            }
        }
    }


    fn get_square_from_mouse(&self, mouse_pos: (f32, f32)) -> (f32, f32) {
        let mouse_x = mouse_pos.0 / (self.square_width as f32);
        let mouse_x = mouse_x.clamp(0f32, (self.squares.len() - 1) as f32);
        let mouse_y = (mouse_pos.1 - (self.top_offset as f32)) / (self.square_width as f32);
        let mouse_y = mouse_y.clamp(0f32, (self.squares[0].len() - 1) as f32);
        (mouse_x, mouse_y)
    }


    pub fn change_square_type(&mut self, mouse_pos: (f32, f32), square_type: SquareType) {
        if mouse_pos.1 > self.top_offset as f32 {
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


    fn manhattan_distance(&self, sq: &(usize, usize)) -> usize {
        let x_dist: isize = sq.0 as isize - self.objective_square.0 as isize;
        let y_dist: isize = sq.1 as isize - self.objective_square.1 as isize;
        let result = (x_dist.abs() + y_dist.abs()) as usize;
        result
    }


    // https://en.wikipedia.org/wiki/A*_search_algorithm
    // true true  -> A*
    // true false -> Dijkstra
    // false true -> Greedy Best first
    pub fn search_algorithm(&mut self, weights: bool, heuristics: bool) {
        self.clear_results();
        if !self.objective_square_set || !self.start_square_set { return }
        let x_dim = self.squares.len() - 1;
        let y_dim = self.squares[0].len() - 1;
        let mut open_set: HashSet<(usize, usize)> = HashSet::new();
        let mut closed_set: HashSet<(usize, usize)> = HashSet::new();
        let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut path_costs: HashMap<(usize, usize), usize> = HashMap::new(); // gScore
        let mut scores: HashMap<(usize, usize), usize> = HashMap::new(); // fScore

        open_set.insert(self.start_square);
        path_costs.insert(self.start_square, 0);
        scores.insert(self.start_square, self.manhattan_distance(&self.start_square));

        'main_loop: while !open_set.is_empty() {
            let mut current = self.get_lowest_score(&scores, &open_set, heuristics);
            if current.0 == self.objective_square.0 && current.1 == self.objective_square.1 {
                loop {
                    current = came_from[&current];
                    if current.0 == self.start_square.0 && current.1 == self.start_square.1 {
                        self.states.push(self.get_current_state());
                        break 'main_loop;
                    }
                    self.squares[current.0][current.1].set_square_type(SquareType::Solution);
                }
            }
            self.squares[current.0][current.1].set_square_type(SquareType::Visited);

            open_set.remove(&current);
            closed_set.insert(current);
            for neighbor in self.neighbors(&current, &x_dim, &y_dim, &closed_set) {
                let mut tentative_path_cost = path_costs[&current];
                if weights {
                    tentative_path_cost += 1;
                }
                if open_set.contains(&neighbor) && tentative_path_cost < path_costs[&neighbor] {
                    came_from.insert(neighbor, current);
                    path_costs.insert(neighbor, tentative_path_cost);
                    if heuristics {
                        tentative_path_cost += self.manhattan_distance(&neighbor);
                    }
                    scores.insert(neighbor, tentative_path_cost);
                }
                if !open_set.contains(&neighbor){
                    open_set.insert(neighbor);
                    came_from.insert(neighbor, current);
                    path_costs.insert(neighbor, tentative_path_cost);
                    if heuristics {
                        tentative_path_cost += self.manhattan_distance(&neighbor);
                    }
                    scores.insert(neighbor, tentative_path_cost);
                }
            }
            self.states.push(self.get_current_state());
        }
        self.current_state = self.states.len() - 1;
    }


    fn neighbors(&self, current: &(usize, usize), x_max: &usize, y_max: &usize, closed_set: &HashSet<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::new();
        if current.1 != 0 {
            let new = (current.0, current.1 - 1);
            if !closed_set.contains(&new) && *self.squares[new.0][new.1].get_square_type() != SquareType::Wall {
                result.push(new);
            }
        }
        if current.1 != *y_max {
            let new = (current.0, current.1 + 1);
            if !closed_set.contains(&new) && *self.squares[new.0][new.1].get_square_type() != SquareType::Wall {
                result.push(new);
            }
        }
        if current.0 != *x_max {
            let new = (current.0 + 1, current.1);
            if !closed_set.contains(&new) && *self.squares[new.0][new.1].get_square_type() != SquareType::Wall {
                result.push(new);
            }
        }
        if current.0 != 0 {
            let new = (current.0 - 1, current.1);
            if !closed_set.contains(&new) && *self.squares[new.0][new.1].get_square_type() != SquareType::Wall {
                result.push(new);
            }
        }
        result
    }


    fn clear_results(&mut self) {
        self.states = Vec::new();
        for x in 0..self.squares.len() {
            for y in 0..self.squares[0].len() {
                let sq = &mut self.squares[x][y];
                let sq_type = sq.get_square_type();
                if *sq_type == SquareType::Visited || *sq_type == SquareType::Solution {
                    self.squares[x][y].set_square_type(SquareType::Blank);
                }
            }
        }
    }


    fn get_lowest_score(&self, scores: &HashMap<(usize, usize), usize>, open_set:&HashSet<(usize, usize)>, dist: bool) -> (usize, usize) {
        let mut result = Vec::from([(0, 0)]);
        let mut minimum = usize::MAX;
        for tuple in open_set {
            if *tuple == self.objective_square {
                return *tuple;
            }
            if scores[tuple] < minimum {
                result = Vec::from([*tuple]);
                minimum = scores[tuple];
            } else if scores[tuple] == minimum {
                result.push(*tuple);
            }
        }
        let mut closest = result.get(0).unwrap();
        if dist {
            minimum = self.manhattan_distance(closest);
            for tuple in &result[1..] {
                let new_dist = self.manhattan_distance(&tuple);
                if new_dist < minimum {
                    minimum = new_dist;
                    closest = &tuple;
                }
            }
        }
        *closest
    }


    pub fn clear(&mut self) {
        for x in 0..self.squares.len() {
            for y in 0..self.squares[0].len() {
                self.squares[x][y].set_square_type(SquareType::Blank);
            }
        }
        self.start_square_set = false;
        self.objective_square_set = false;
    }


    pub fn generate_maze(&mut self) {
        let mut rng = rand::thread_rng();
        for x in 0..self.squares.len() {
            for y in 0..self.squares[0].len() {
                let test = (rng.gen::<f32>() * 5f32) as usize;
                if test % 5 > 1 {
                    self.squares[x][y].set_square_type(SquareType::Blank);
                } else {
                    self.squares[x][y].set_square_type(SquareType::Wall);
                }
            }
        }
        self.start_square_set = false;
        self.objective_square_set = false;
    }


    fn get_current_state(&self) -> Vec<Vec<SquareType>> {
        let mut result: Vec<Vec<SquareType>> = Vec::new();
        for x in 0..self.squares.len() {
            let mut col: Vec<SquareType> = Vec::new();
            for y in 0..self.squares[0].len() {
                col.push(self.squares[x][y].square_type.to_owned())
            }
            result.push(col);
        }
        result
    }


    pub fn load_next_state(&mut self) {
        if self.states.len() > 0 {
            self.current_state += 1;
            if self.current_state == self.states.len() {
                self.current_state = 0;
            }
            self.load_state();
        }
    }


    pub fn load_previous_state(&mut self) {
        if self.states.len() > 0 {
            if self.current_state == 0 {
                self.current_state = self.states.len() - 1;
            } else {
                self.current_state -= 1;
            }
            self.load_state();
        }
    }


    fn load_state(&mut self) {
        for x in 0..self.squares.len() {
            for y in 0..self.squares[0].len() {
                self.squares[x][y].set_square_type(self.states[self.current_state][x][y].to_owned());
            }
        }
    }
}
