use core::fmt;
use std::collections::{btree_map, BTreeMap};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NumberList {
    frequencies: BTreeMap<i32, i32>,
    len: usize,
}

impl NumberList {
    pub fn new(nums: &Vec<i32>) -> Self {
        let mut list = NumberList {
            frequencies: BTreeMap::new(),
            len: nums.len(),
        };

        for i in nums {
            list.frequencies
                .entry(*i)
                .and_modify(|freq| *freq += 1)
                .or_insert(1);
        }

        list
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn unique_pairs(&self) -> NumberPairIter {
        NumberPairIter {
            outer: self.frequencies.iter(),
            curr_outer: None,
            inner: self.frequencies.iter(),
        }
    }

    pub fn remove(&mut self, num: i32) {
        assert!(self.frequencies.contains_key(&num));
        if *self.frequencies.get(&num).unwrap() == 1 {
            self.frequencies.remove(&num);
        } else {
            self.frequencies.entry(num).and_modify(|e| *e -= 1);
        }

        self.len -= 1;
    }

    pub fn add(&mut self, num: i32) {
        self.frequencies
            .entry(num)
            .and_modify(|e| *e += 1)
            .or_insert(1);
        self.len += 1;
    }

    pub fn replace_pair(&mut self, pair: (i32, i32), replacement: i32) {
        self.remove(pair.0);
        self.remove(pair.1);
        self.add(replacement);
    }
}

impl<'a> IntoIterator for &'a NumberList {
    type IntoIter = NumberListIter<'a>;
    type Item = i32;
    fn into_iter(self) -> Self::IntoIter {
        NumberListIter {
            iter: self.frequencies.iter(),
            curr_entry: None,
            curr_count: 0,
        }
    }
}

impl fmt::Display for NumberList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "NumberList {{")?;
		let mut iter = self.into_iter();
		if let Some(val) = iter.next() {
			write!(f, "{}", val)?;
			for v in iter {
				write!(f, ", {}", v)?;
			}
		}
		write!(f, "}}")?;
		Ok(())
	}
}

#[derive(Clone, Debug)]
pub struct NumberListIter<'a> {
    iter: btree_map::Iter<'a, i32, i32>,
    curr_entry: Option<(i32, i32)>,
    curr_count: i32,
}

impl<'a> Iterator for NumberListIter<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_entry.is_none() {
            if let Some(entry) = self.iter.next() {
                self.curr_entry = Some((*entry.0, *entry.1));
            } else {
                return None;
            }
        }
        let entry = self.curr_entry.unwrap();
        if self.curr_count < entry.1 {
            self.curr_count += 1;
            return Some(entry.0);
        }

        if let Some(entry) = self.iter.next() {
            self.curr_entry = Some((*entry.0, *entry.1));
            self.curr_count = 0;
            return self.next();
        }
        None
    }
}

#[derive(Clone, Debug)]
pub struct NumberPairIter<'a> {
    outer: btree_map::Iter<'a, i32, i32>,
    curr_outer: Option<i32>,
    inner: btree_map::Iter<'a, i32, i32>,
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
            Some((k, _v)) => Some((self.curr_outer.unwrap(), *k)),
            None => match self.outer.next() {
                Some((k, _v)) => {
                    self.curr_outer = Some(*k);
                    self.inner = self.outer.clone();
                    self.next()
                }
                None => None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let mut nums = vec![3, 4, 4, 1, 1, 7, 6, 12, 12, 6, 9, 9, 1, 9, 9];
        let list = NumberList::new(&nums);
        nums.sort();

        assert_eq!(list.len(), nums.len());
        for (i, v) in list.into_iter().enumerate() {
            assert_eq!(v, nums[i]);
        }
    }

    #[test]
    fn test_unique_pairs() {
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
            (6, 7),
        ];

        assert_eq!(list.len(), 5);
        for (i, p) in list.unique_pairs().enumerate() {
            assert_eq!(p, expected[i]);
        }

        let list = NumberList::new(&vec![6, 6, 3, 3, 3, 7, 1, 1, 4, 4, 4, 4]);

        assert_eq!(list.len(), 12);
        for (i, p) in list.unique_pairs().enumerate() {
            assert_eq!(p, expected[i]);
        }
    }

    #[test]
    fn tests_add_remove() {
        let mut list = NumberList::new(&vec![6, 6, 3, 3, 3, 7, 1, 1, 4, 4, 4, 4]);
        list.remove(6);
        list.remove(3);
        list.remove(3);
        list.remove(1);
        list.remove(4);
        list.remove(4);
        list.remove(4);

        let expected = vec![1, 3, 4, 6, 7];
        assert_eq!(list.len(), expected.len());
        for (i, v) in list.into_iter().enumerate() {
            assert_eq!(v, expected[i]);
        }

        let mut list = NumberList::new(&vec![6, 6, 3, 3, 3, 7, 1, 1, 4, 4, 4, 4]);
        list.replace_pair((6, 3), 8);
        list.replace_pair((6, 3), 7);
        list.replace_pair((1, 4), 5);
        list.replace_pair((4, 4), 5);
        list.replace_pair((5, 7), 10);
        let expected = vec![1, 3, 4, 5, 7, 8, 10];
        assert_eq!(list.len(), expected.len());
        for (i, v) in list.into_iter().enumerate() {
            assert_eq!(v, expected[i]);
        }
    }
}
