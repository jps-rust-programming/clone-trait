// pub mod Shape;

use super::shape::Shape;

pub trait Colorful: Shape {
    fn color(&self) -> &str;
}
