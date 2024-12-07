use core::{fmt, slice};
use std::{collections::HashMap, ops::Range};

use crate::{number_list::NumberList, op::Reduce};

pub type NodeData = NumberList;
pub type EdgeData = Reduce;
pub type NodeId = u32;
pub type EdgeId = u32;

pub struct Edge {
    src: NodeId,
    dst: NodeId,
    data: EdgeData,
}

impl Edge {
    pub fn src(&self) -> NodeId {
        self.src
    }
    pub fn dst(&self) -> NodeId {
        self.dst
    }
    pub fn data(&self) -> &EdgeData {
        &self.data
    }
}

pub struct Node {
    data: NodeData,
    // only contains edges starting from this node
    edges: Vec<Edge>,
}

pub struct Graph {
    nodes: Vec<Node>,
    node_map: HashMap<NodeData, NodeId>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            node_map: HashMap::new(),
        }
    }

    pub fn node_id(&self, data: &NodeData) -> Option<NodeId> {
        if let Some(id) = self.node_map.get(data) {
            Some(*id)
        } else {
            None
        }
    }

    pub fn node_data(&self, id: NodeId) -> &NodeData {
        &self.nodes[id as usize].data
    }

    pub fn num_nodes(&self) -> u32 {
        self.nodes.len() as u32
    }

    pub fn all_nodes(&self) -> Range<NodeId> {
        0..self.num_nodes()
    }

    pub fn node_edges(&self, id: NodeId) -> EdgeIter {
        EdgeIter {
            iter: self.nodes[id as usize].edges.iter(),
        }
    }

    pub fn get_or_add_node(&mut self, data: &NodeData) -> NodeId {
        if let Some(id) = self.node_id(data) {
            id
        } else {
            self.add_node(data)
        }
    }

    pub fn add_node(&mut self, data: &NodeData) -> NodeId {
        let id = self.nodes.len() as NodeId;
        self.nodes.push(Node {
            data: data.clone(),
            edges: Vec::new(),
        });
        self.node_map.insert(data.clone(), id);

        id
    }

    pub fn add_edge(&mut self, src: NodeId, dst: NodeId, data: &EdgeData) {
        self.nodes[src as usize].edges.push(Edge {
            src: src,
            dst: dst,
            data: data.clone(),
        });
    }
}

pub struct EdgeIter<'a> {
    iter: slice::Iter<'a, Edge>,
}

impl<'a> Iterator for EdgeIter<'a> {
    type Item = &'a Edge;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct Dot<'a> {
    graph: &'a Graph,
}

impl<'a> Dot<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        Self { graph: graph }
    }
}

impl<'a> fmt::Display for Dot<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let g = self.graph;
        writeln!(f, "digraph {{")?;
        for node_id in g.all_nodes() {
            writeln!(f, "    {} [label=\"{}\"]", node_id, g.node_data(node_id))?;
        }
        for node_id in g.all_nodes() {
            for edge in g.node_edges(node_id) {
                writeln!(
                    f,
                    "    {} -> {} [label=\"{}\"]",
                    edge.src, edge.dst, edge.data
                )?;
            }
        }
        writeln!(f, "}}")
    }
}
