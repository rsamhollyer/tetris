use std::{collections::HashSet, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(pub i32, pub i32);

impl Add for Position {
    type Output = Position;
    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}
#[derive(Debug, Clone)]
pub struct Shape {
    positions: HashSet<Position>,
    anchor: Position,
}

macro_rules! impl_shape_constructor {
    ($( $new:ident: [$($pos:expr),*] anchored at $anchor:expr; )*) => {
       $(
        pub fn $new() -> Self {
            Self {
                positions: [$($pos),*].into_iter().collect(),
                anchor: $anchor,
            }
        })*
    };
}

impl Shape {
    impl_shape_constructor! {
        new_i: [Position(0, 0),Position(1, 0),Position(2, 0),Position(3, 0)] anchored at Position(1, 0);
        new_o:[Position(0, 0),Position(1, 0),Position(0, 1),Position(1, 1)] anchored at Position(0, 0);
        new_t:[Position(0, 0),Position(1, 0),Position(2, 0),Position(1, 1)] anchored at Position(0, 0);
        new_j:[Position(0, 0),Position(0, 1),Position(0, 2),Position(-1, 2)] anchored at Position(0, 1);
        new_l:[Position(0, 0),Position(0, 1),Position(0, 2),Position(1, 2)] anchored at Position(0, 1);
        new_s:[Position(0, 0),Position(1, 0),Position(0, 1),Position(-1, 1)] anchored at Position(0, 0);
        new_z:[Position(0, 0),Position(-1, 0),Position(0, 1),Position(1, 1)] anchored at Position(0, 0);
    }

    pub fn new_random() -> Self {
        let random = (rand::random::<f64>() * 7.0).floor() as u8;

        match random {
            0 => Self::new_i(),
            1 => Self::new_o(),
            2 => Self::new_t(),
            3 => Self::new_j(),
            4 => Self::new_l(),
            5 => Self::new_s(),
            6 => Self::new_z(),
            _ => unreachable!(),
        }
    }
}

impl Add<Position> for &Shape {
    type Output = Shape;

    fn add(self, rhs: Position) -> Self::Output {
        Shape {
            positions: self.positions.iter().map(|&pos| pos + rhs).collect(),
            anchor: self.anchor + rhs,
        }
    }
}
