use crate::utils::Point;
use crossterm::{
    cursor::{Hide, MoveTo},
    style::Print,
    terminal::Clear,
    ExecutableCommand,
};
use std::io::stdout;

use super::display::Display;

pub struct ConsoleDisplay {
    screen_x: u16,
    screen_y: u16,
}

impl ConsoleDisplay {
    pub fn new(x: u16, y: u16) -> Self {
        stdout().execute(Hide).unwrap();

        ConsoleDisplay {
            screen_x: x,
            screen_y: y,
        }
    }
}

impl Display for ConsoleDisplay {
    fn put(&self, point: &Point, x: u16, y: u16) {
        if x >= self.screen_x || y >= self.screen_y {
            return;
        }
        stdout().execute(MoveTo(x, y)).unwrap();

        stdout().execute(Print(point.get_char())).unwrap();
    }

    fn clear(&self) {
        stdout()
            .execute(Clear(crossterm::terminal::ClearType::All))
            .unwrap();

        for y in 0..self.screen_y {
            for x in 0..self.screen_x {
                self.put(&Point::C, x, y);
            }
        }
    }
}
