use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(pub i32, pub i32);

#[derive(Debug, Clone)]
pub struct Shape {
    positions: HashSet<Position>,
}

pub struct Tetris {
    width: usize,
    height: usize,
    current_shape: Shape,
    fixed_shapes: Vec<Shape>,
}
