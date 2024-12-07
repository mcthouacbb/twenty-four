use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    // will need some fraction type for division
    //Div
}

impl Operation {
    pub fn apply(&self, left: i32, right: i32) -> i32 {
        match self {
            Self::Add => left + right,
            Self::Sub => left - right,
            Self::Mul => left * right,
        }
    }

    pub fn all_ops() -> [Self; 3] {
        [Self::Add, Self::Sub, Self::Mul]
    }

    pub fn commutative(&self) -> bool {
        *self == Self::Add || *self == Self::Mul
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Reduce {
    left: i32,
    right: i32,
    op: Operation,
}

impl Reduce {
    pub fn new(left: i32, right: i32, op: Operation) -> Self {
        Self {
            left: left,
            right: right,
            op: op,
        }
    }
}

impl fmt::Display for Reduce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} = {}",
            self.left,
            self.op,
            self.right,
            self.op.apply(self.left, self.right)
        )
    }
}
