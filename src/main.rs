use display::{console_display::ConsoleDisplay, display::Display};
use font::{RenderComponent, TestLetter};
use std::{thread::sleep, time::Duration};
use utils::Coord;

mod display;
mod font;
mod utils;

struct Runner {
    display: Box<dyn Display>,
    actions: Vec<Box<dyn RenderComponent>>,
}

impl Runner {
    fn new(size: Coord) -> Self {
        Runner {
            display: Box::new(ConsoleDisplay::new(size.x, size.y)),
            actions: vec![Box::new(TestLetter::new())],
        }
    }
    fn infinite_loop(&mut self) {
        self.display.clear();

        loop {
            for action in self.actions.iter_mut() {
                action.draw(&self.display);
                action.update();
            }

            sleep(Duration::from_millis(40));
        }
    }
}

fn main() {
    let mut runner = Runner::new(Coord::new(100, 10));

    runner.infinite_loop();
}
