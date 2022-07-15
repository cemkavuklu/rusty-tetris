use crate::shape::{Pos, Shape};

#[derive(Debug)]
pub struct Tetris {
    width: u32,
    height: u32,
    current_shape: Shape,
    fixed_shapes: Vec<Shape>,
}

impl Tetris {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            current_shape: &Shape::new_random() + Pos((width / 2) as i32, 0),
            fixed_shapes: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tetris;

    #[test]
    fn test() {
        println!("{:#?}", Tetris::new(100, 10));
    }
}
