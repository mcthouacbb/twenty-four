use std::{collections::HashSet, mem::swap};

use crate::number_list::NumberList;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add,
    Sub,
    Mul,
    // will need some fraction type for division
    //Div
}

impl Operation {
    fn apply(&self, left: i32, right: i32) -> i32 {
        match self {
            Operation::Add => left + right,
            Operation::Sub => left - right,
            Operation::Mul => left * right,
        }
    }

    fn all_ops() -> [Operation; 3] {
        [Operation::Add, Operation::Sub, Operation::Mul]
    }

    fn commutative(&self) -> bool {
        *self == Operation::Add || *self == Operation::Mul
    }
}

struct Reduce {
    left: i32,
    right: i32,
    op: Operation,
}

#[derive(Clone, Debug)]
pub struct Solver {
    base: NumberList,
}

impl Solver {
    pub fn new(base: NumberList) -> Self {
        Solver { base: base }
    }

    pub fn solve(&self) {
        let mut prev_layer = HashSet::from([self.base.clone()]);
        let mut layer: HashSet<NumberList> = HashSet::new();

        for _ in 0..(self.base.len() - 1) {
            layer.clear();
            for num_list in &prev_layer {
                for pair in num_list.unique_pairs() {
                    for op in Operation::all_ops() {
                        let mut reduced = num_list.clone();
                        reduced.replace_pair(pair, op.apply(pair.0, pair.1));
                        layer.insert(reduced);

                        if !op.commutative() {
                            let mut reduced = num_list.clone();
                            reduced.replace_pair(pair, op.apply(pair.1, pair.0));
                            layer.insert(reduced);
                        }
                    }
                }
            }

            swap(&mut prev_layer, &mut layer);
        }
    }
}
