use std::collections::HashSet;

use crate::number_list::NumberList;

enum Operation {
	Add,
	Sub,
	Mul,
	// will need some fraction type for division
	//Div
}

struct Reduce {
	left: i32,
	right: i32,
	op: Operation
}

#[derive(Clone, Debug)]
pub struct Solver {
	base: NumberList
}

impl Solver {
	pub fn new(base: NumberList) -> Self {
		Solver {
			base: base
		}
	}

	pub fn solve(&self) {
		let prev_layer = HashSet::from([self.base.clone()]);
		let layer: HashSet<NumberList> = HashSet::new();

		loop {
			for num_list in &prev_layer {
				for pair in num_list.unique_pairs() {
					
				}
			}
		}
	}
}
