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
    typ: &'static str,
    positions: HashSet<Position>,
    anchor: Position,
}

macro_rules! impl_shape_constructor {
    ($( $new:ident $typ:literal: [$($pos:expr),*] @ $anchor:expr; )*) => {
       $(
        pub fn $new() -> Self {
            Self {
                typ: $typ,
                positions: [$($pos),*].into_iter().collect(),
                anchor: $anchor,
            }
        })*
    };
}

impl Shape {
    impl_shape_constructor! {
        new_i "I" : [Position(0, 0),Position(1, 0),Position(2, 0),Position(3, 0)] @ Position(1, 0);
        new_o "O" : [Position(0, 0),Position(1, 0),Position(0, 1),Position(1, 1)] @ Position(0, 0);
        new_t "T" : [Position(0, 0),Position(1, 0),Position(2, 0),Position(1, 1)] @ Position(0, 0);
        new_j "J" : [Position(0, 0),Position(0, 1),Position(0, 2),Position(-1, 2)] @ Position(0, 1);
        new_l "L" : [Position(0, 0),Position(0, 1),Position(0, 2),Position(1, 2)] @ Position(0, 1);
        new_s "S" : [Position(0, 0),Position(1, 0),Position(0, 1),Position(-1, 1)] @ Position(0, 0);
        new_z "Z" : [Position(0, 0),Position(-1, 0),Position(0, 1),Position(1, 1)] @ Position(0, 0);
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

    pub fn typ(&self) -> &'static str {
        self.typ
    }

    pub fn iter_positions(&self) -> impl Iterator<Item = Position> + '_ {
        self.positions.iter().copied()
    }

    pub fn has_position(&self, pos: Position) -> bool {
        self.positions.contains(&pos)
    }

    pub fn collides_with(&self, other: &Shape) -> bool {
        self.positions.intersection(&other.positions).count() > 0
    }

    pub fn rotated(&self) -> Self {
        let Position(a, b) = self.anchor;

        Self {
            typ: self.typ,
            positions: self
                .iter_positions()
                .map(|Position(x, y)| Position(-y + b + a, x - a + b))
                .collect(),
            anchor: self.anchor,
        }
    }

    pub fn remove_line(&mut self, y: i32) {
        self.positions = self
            .positions
            .iter()
            .copied()
            .filter(|pos| pos.1 != y)
            .map(|pos| {
                if pos.1 >= y {
                    pos
                } else {
                    Position(pos.0, pos.1 + 1)
                }
            })
            .collect();
    }
}

impl Add<Position> for &Shape {
    type Output = Shape;

    fn add(self, rhs: Position) -> Self::Output {
        Shape {
            typ: self.typ,
            positions: self.positions.iter().map(|&pos| pos + rhs).collect(),
            anchor: self.anchor + rhs,
        }
    }
}
