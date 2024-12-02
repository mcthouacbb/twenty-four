use std::{collections::BTreeMap, collections::btree_map};

pub struct NumberList {
	frequencies: BTreeMap<i32, i32>,
	len: usize
}

impl NumberList {
	pub fn new(nums: &Vec<i32>) -> Self {
		let mut list = NumberList {
			frequencies: BTreeMap::new(),
			len: nums.len()
		};

		for i in nums {
			list.frequencies.entry(*i).and_modify(|freq| *freq += 1).or_insert(1);
		}

		list
	}

	pub fn len(&self) -> usize {
		self.len
	}

	pub fn pairs(&self) -> NumberPairIter {
		NumberPairIter {
			outer: self.frequencies.iter(),
			curr_outer: None,
			inner: self.frequencies.iter()
		}
	}
}

pub struct NumberPairIter<'a> {
	outer: btree_map::Iter<'a, i32, i32>,
	curr_outer: Option<i32>,
	inner: btree_map::Iter<'a, i32, i32>
}

impl<'a> Iterator for NumberPairIter<'a> {
	type Item = (i32, i32);
	fn next(&mut self) -> Option<Self::Item> {
		if self.curr_outer.is_none() {
			match self.outer.next() {
				Some((k, _v)) => {
					self.curr_outer = Some(*k);
					self.inner = self.outer.clone()
				}
				None => {
					return None;
				}
			}
		}
		match self.inner.next() {
			Some((k, _v)) => {
				Some((self.curr_outer.unwrap(), *k))
			}
			None => {
				match self.outer.next() {
					Some((k, _v)) => {
						self.curr_outer = Some(*k);
						self.inner = self.outer.clone();
						self.next()
					}
					None => {
						None
					}
				}
			}
		}
	}
}

pub struct Solver {

}

impl Solver {

}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_num() {
		let list = NumberList::new(&vec![3, 7, 4, 1, 6]);
		let expected = [
			(1, 3),
			(1, 4),
			(1, 6),
			(1, 7),
			(3, 4),
			(3, 6),
			(3, 7),
			(4, 6),
			(4, 7),
			(6, 7)
		];

		for (i, p) in list.pairs().enumerate() {
			assert_eq!(p, expected[i]);
		}

		let list = NumberList::new(&vec![6, 6, 3, 3, 3, 7, 1, 1, 4, 4, 4, 4]);

		for (i, p) in list.pairs().enumerate() {
			assert_eq!(p, expected[i]);
		}
	}
}
