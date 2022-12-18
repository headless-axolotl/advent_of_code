use std::fmt::Debug;


#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum State { Air, Rock }
type S = State;
pub type Shape = [[S; 4]; 4];

pub const HORIZONTAL: Shape = [
    [S::Rock, S::Rock, S::Rock, S::Rock],
    [S::Air , S::Air , S::Air , S::Air ],
    [S::Air , S::Air , S::Air , S::Air ],
    [S::Air , S::Air , S::Air , S::Air ],
];

pub const PLUS: Shape = [
    [S::Air , S::Rock, S::Air , S::Air ],
    [S::Rock, S::Rock, S::Rock, S::Air ],
    [S::Air , S::Rock, S::Air , S::Air ],
    [S::Air , S::Air , S::Air , S::Air ],
];

pub const LSHAPE: Shape = [
    [S::Rock, S::Rock, S::Rock, S::Air ],
    [S::Air , S::Air , S::Rock, S::Air ],
    [S::Air , S::Air , S::Rock, S::Air ],
    [S::Air , S::Air , S::Air , S::Air ],
];

pub const VERTICAL: Shape = [
    [S::Rock, S::Air , S::Air , S::Air ],
    [S::Rock, S::Air , S::Air , S::Air ],
    [S::Rock, S::Air , S::Air , S::Air ],
    [S::Rock, S::Air , S::Air , S::Air ],
];

pub const CUBE: Shape = [
    [S::Rock, S::Rock, S::Air , S::Air ],
    [S::Rock, S::Rock, S::Air , S::Air ],
    [S::Air , S::Air , S::Air , S::Air ],
    [S::Air , S::Air , S::Air , S::Air ],
];

pub fn get_rock_shape(index: usize) -> Shape {
    match index % 5 {
        0 => HORIZONTAL,
        1 => PLUS,
        2 => LSHAPE,
        3 => VERTICAL,
        4 => CUBE,
        _ => CUBE
    }
}

pub fn get_rock_height(index: usize) -> usize {
    match index % 5 {
        0 => 1,
        1 => 3,
        2 => 3,
        3 => 4,
        4 => 2,
        _ => 2
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Air => write!(f, "."),
            Self::Rock => write!(f, "#"),
        }
    }
}