use std::fmt::{Debug, Write};

use crate::display::display::Display;

#[derive(Clone, Copy)]
pub enum Point {
    F,
    C,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.get_char())
    }
}

impl Point {
    pub fn from_char(char: &char) -> Self {
        match char {
            '#' => Point::F,
            '-' => Point::C,
            _ => Point::C,
        }
    }

    pub fn get_char(&self) -> char {
        match self {
            Point::F => '#',
            Point::C => '-',
        }
    }
}

#[derive(Clone)]
pub struct Coord {
    pub x: u16,
    pub y: u16,
}

impl Coord {
    pub fn new(x: u16, y: u16) -> Self {
        Coord { x, y }
    }
}

#[derive(Clone)]
pub struct Vertex {
    pub points: Vec<Vec<Point>>,
    pub top_left: Coord,
}

impl Vertex {
    pub fn new(points: Vec<Vec<Point>>, top_left: Coord) -> Self {
        Vertex { points, top_left }
    }

    pub fn from_string(map: String, top_left: Coord) -> Self {
        let mut points = Vec::new();

        for line in map.trim().lines() {
            let mut temp_line = Vec::new();
            for char in line.trim_ascii().chars() {
                temp_line.push(Point::from_char(&char));
            }
            points.push(temp_line);
        }

        Vertex::new(points, top_left)
    }

    pub fn draw(&self, display: &Box<dyn Display>) {
        for y in 0..self.points.len() as u16 {
            for x in 0..self.points[y as usize].len() as u16 {
                display.put(
                    &self.points[y as usize][x as usize],
                    x + self.top_left.x,
                    y + self.top_left.y,
                );
            }
        }
    }
}
