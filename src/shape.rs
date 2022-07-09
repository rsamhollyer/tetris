use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(pub i32, pub i32);

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
}
