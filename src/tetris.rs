use crate::shape::{Position, Shape};

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
            current_shape: &Shape::new_random() + Position((width as i32) / 2, 0),
            fixed_shapes: vec![],
        }
    }

    pub fn is_out_of_bounds($self,shape:&Shape)->bool{
        shape.positions.iter().any(|&pos|{
            pos.0 < 0 || pos.0 >= $self.width as i32 || pos.1 >= $self.height as i32
        })
    }

    pub fn tick(&mut self) {
        let translated_current_shape = &self.current_shape + Position(0, 1);
    }
}

#[cfg(test)]
mod tests {
    use super::Tetris;
    #[test]
    fn test() {
        let mut tetris = Tetris::new(10, 30);
        tetris.tick();
        tetris.tick();
        tetris.tick();
        tetris.tick();

        println!("{:?}", tetris);
    }
}
