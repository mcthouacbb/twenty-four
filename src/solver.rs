use core::fmt;
use std::collections::HashSet;

use crate::graph::Graph;
use crate::number_list::NumberList;
use crate::op::{Operation, Reduce};

#[derive(Clone, Debug)]
pub struct Solver {
    base: NumberList,
}

impl Solver {
    pub fn new(base: NumberList) -> Self {
        Solver { base: base }
    }

    pub fn solve(&self) {
        let mut graph = Graph::new();
        let root = graph.add_node(&self.base);
        let mut layer = HashSet::new();
        let mut prev_layer = HashSet::from([root]);

        for _ in 0..(self.base.len() - 1) {
            layer.clear();
            for node_idx in &prev_layer {
                let num_list = graph.node_data(*node_idx).clone();
                for pair in num_list.unique_pairs() {
                    for op in Operation::all_ops() {
                        let mut reduced = num_list.clone();
                        reduced.replace_pair(pair, op.apply(pair.0, pair.1));
                        let new_node_idx = graph.get_or_add_node(&reduced);
                        layer.insert(new_node_idx);
                        graph.add_edge(*node_idx, new_node_idx, &Reduce::new(pair.0, pair.1, op));

                        if !op.commutative() {
                            let mut reduced = num_list.clone();
                            reduced.replace_pair(pair, op.apply(pair.1, pair.0));
                            let new_node_idx = graph.get_or_add_node(&reduced);
                            layer.insert(new_node_idx);
                            graph.add_edge(
                                *node_idx,
                                new_node_idx,
                                &Reduce::new(pair.1, pair.0, op),
                            );
                        }
                    }
                }
            }

            std::mem::swap(&mut prev_layer, &mut layer);
        }
    }
}
