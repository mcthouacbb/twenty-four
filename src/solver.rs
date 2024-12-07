use core::fmt;
use std::collections::HashSet;
use std::process::id;

use crate::graph::{Dot, Graph, NodeId};
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

        // println!("{}", Dot::new(&graph));

        let mut reach_map = vec![false; graph.num_nodes() as usize];

        Self::check_reachability(&graph, root, &mut reach_map, 24);
        if reach_map[root as usize] {
            println!("Can reach 24");
        } else {
            println!("Cannot reach 24");
        }

        let pruned = Self::prune_graph(&graph, &reach_map);
        println!("{}", Dot::new(&pruned));
    }

    fn check_reachability(
        graph: &Graph,
        curr_node: NodeId,
        reach_map: &mut Vec<bool>,
        target: i32,
    ) -> bool {
        let data = graph.node_data(curr_node);
        if data.len() == 1 {
            let reaches = data.single_val() == target;
            reach_map[curr_node as usize] = reaches;
            return reaches;
        }
        let mut reaches = false;
        for edge in graph.node_edges(curr_node) {
            reaches |= Self::check_reachability(graph, edge.dst(), reach_map, target);
        }
        reach_map[curr_node as usize] = reaches;
        reaches
    }

    fn prune_graph(graph: &Graph, reach_map: &Vec<bool>) -> Graph {
        let mut id_map = vec![0 as NodeId; reach_map.len()];
        let mut pruned = Graph::new();
        for (id, reaches) in reach_map.iter().enumerate() {
            if *reaches || id == 0 {
                id_map[id] = pruned.add_node(graph.node_data(id as NodeId));
            }
        }
        for (id, reaches) in reach_map.iter().enumerate() {
            if *reaches {
                for edge in graph.node_edges(id as NodeId) {
                    if reach_map[edge.dst() as usize] {
                        pruned.add_edge(
                            id_map[edge.src() as usize],
                            id_map[edge.dst() as usize],
                            edge.data(),
                        )
                    }
                }
            }
        }

        pruned
    }
}
