use crate::edge::Edge;
use crate::node::Node;
use attributes::{Attributes, AttributesContainer};
use attributes_derive::AttributesContainer;

pub mod graph_items {
    pub mod node {
        pub use crate::node::Node;
    }
    pub mod edge {
        pub use crate::edge::Edge;
    }
}

#[derive(Clone, Debug, PartialEq, AttributesContainer)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    #[attributes]
    pub attrs: Attributes,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            attrs: Attributes::new(),
        }
    }

    pub fn with_nodes<T>(mut self, nodes: T) -> Self
    where
        T: AsRef<[Node]>,
    {
        self.nodes.extend(nodes.as_ref().iter().cloned());
        self
    }

    pub fn with_edges<T>(mut self, edges: T) -> Self
    where
        T: AsRef<[Edge]>,
    {
        self.edges.extend(edges.as_ref().iter().cloned());
        self
    }

    pub fn get_node<T>(&self, name: T) -> Option<&Node>
    where
        T: Into<String>,
    {
        let name = name.into();
        self.nodes.iter().find(|&node| node.name == name)
    }
}
