use crate::{
    display::display::Display,
    utils::{Coord, Point, Vertex},
};

fn clean_vertex(prev_vertex: &Vertex, display: &Box<dyn Display>) {
    let mut vertex = prev_vertex.clone();

    for i in vertex.points.iter_mut() {
        for j in i.iter_mut() {
            *j = Point::C;
        }
    }

    vertex.draw(display);
}

pub trait RenderComponent {
    fn draw(&self, display: &Box<dyn Display>);
    fn update(&mut self);
}

pub struct TestLetter {
    vertex: Vertex,
    prev_vertex: Option<Vertex>,
}

impl TestLetter {
    pub fn new() -> Self {
        let vertex = "
##------##
-##----##-
--##--##--
---####---
----##----
---####---
--##--##--
-##----##-
##------##"
            .to_string();
        TestLetter {
            vertex: Vertex::from_string(vertex, Coord::new(0, 0)),
            prev_vertex: None,
        }
    }
}

impl RenderComponent for TestLetter {
    fn draw(&self, display: &Box<dyn Display>) {
        if let Some(vertex) = &self.prev_vertex {
            clean_vertex(vertex, &display);
        }

        self.vertex.draw(&display);
    }

    fn update(&mut self) {
        self.prev_vertex = Some(self.vertex.clone());
        self.vertex.top_left.x += 1;
    }
}
