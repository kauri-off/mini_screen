use crate::utils::Point;

pub trait Display {
    fn put(&self, point: &Point, x: u16, y: u16);
    fn clear(&self);
}
