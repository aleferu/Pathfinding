#![allow(unused)]

use macroquad::prelude as mq;
use std::collections::{HashMap, HashSet};

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
    x_grid: usize,
    y_grid: usize,
    square_type: SquareType,
}

impl Square {
    pub fn new(x: usize, y: usize) -> Square {
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

    pub fn get_x(&self) -> usize {
        self.x_grid as usize
    }

    pub fn get_y(&self) -> usize {
        self.y_grid as usize
    }

    pub fn equals(&self, other: &Square) -> bool{
        self.x_grid == other.x_grid && self.y_grid == other.y_grid
    }

    fn draw(&self, square_width: usize, top_offset: usize) {
        let x_coord = self.x_grid * square_width;
        let y_coord = self.y_grid * square_width + top_offset;
        let color = match self.square_type {
            SquareType::Blank => mq::WHITE,
            SquareType::Wall=> mq::BLACK,
            SquareType::Start=> mq::GREEN,
            SquareType::Objective => mq::RED,
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
    start_square_set: bool
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
            start_square_set: false
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

    pub fn is_start_square_set(&self) -> bool {
        self.start_square_set
    }

    pub fn is_objective_square_set(&self) -> bool {
        self.objective_square_set
    }

    fn manhattan_distance(&self, sq: (usize, usize)) -> usize {
        let x_dist: isize = sq.0 as isize - self.objective_square.0 as isize;
        let y_dist: isize = sq.1 as isize - self.objective_square.1 as isize;
        (x_dist.abs() + y_dist.abs()) as usize
    }

// https://en.wikipedia.org/wiki/A*_search_algorithm
    // I'm not in the mood to handle HashMap in this language atm
    pub fn a_star(&mut self) {
        let x_dim = self.squares.len() - 1;
        let y_dim = self.squares[0].len() - 1;
        println!("{x_dim} {y_dim}");
        let mut open_set: HashSet<(usize, usize)> = HashSet::new();
        let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut path_costs: HashMap<(usize, usize), usize> = HashMap::new(); // gScore
        let mut scores: HashMap<(usize, usize), usize> = HashMap::new(); // fScore

        open_set.insert(self.start_square);
        path_costs.insert(self.start_square, 0);
        scores.insert(self.start_square, self.manhattan_distance(self.start_square));

        while !open_set.is_empty() {
            let mut current = get_lowest_score(&scores, &open_set);
            if current.0 == self.objective_square.0 && current.1 == self.objective_square.1 {
                'reconstruct_path: loop {
                    current = came_from[&current];
                    if current.0 == self.start_square.0 && current.1 == self.start_square.1 {
                        break;
                    }
                    self.squares[current.0][current.1].got_visited();
                }
            }

            open_set.remove(&current);
            // WIP, DOESN'T WORK
            for neighbor in neighbors(&current, &x_dim, &y_dim) {
                let tentative_path_cost = path_costs[&current] + 1;
                println!("{path_costs:?}, {neighbor:?}");
                if open_set.contains(&neighbor) && tentative_path_cost < path_costs[&neighbor] {
                    came_from.insert(neighbor, current);
                    path_costs.insert(neighbor, tentative_path_cost);
                    scores.insert(neighbor, tentative_path_cost + self.manhattan_distance(neighbor));
                }
                if !open_set.contains(&neighbor) {
                    open_set.insert(neighbor);
                    came_from.insert(neighbor, current);
                    path_costs.insert(neighbor, tentative_path_cost);
                    scores.insert(neighbor, tentative_path_cost + self.manhattan_distance(neighbor));
                }
            }
        }
    }
}

fn get_lowest_score(scores: &HashMap<(usize, usize), usize>, open_set:&HashSet<(usize, usize)>) -> (usize, usize) {
    let mut result = (0, 0);
    let minimum = 0usize; // scores are >= 1
    for tuple in open_set {
        if scores[tuple] > minimum {
            result = *tuple;
        }
    }
    result
}

fn neighbors(current: &(usize, usize), x_max: &usize, y_max: &usize) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    if current.0 != 0 && current.1 != 0 {
        result.push((current.0 - 1, current.1 - 1));
    }
    if current.0 != 0 && current.1 != *y_max {
        result.push((current.0 - 1, current.1 + 1));
    }
    if current.0 != *x_max && current.1 != 0 {
        result.push((current.0 + 1, current.1 - 1));
    }
    if current.0 != *x_max && current.1 != *y_max {
        result.push((current.0 + 1, current.1 + 1));
    }
    result
}
