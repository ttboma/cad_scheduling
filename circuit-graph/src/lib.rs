use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircuitGraph {
    input_pins: Vec<String>,
    output_pins: Vec<String>,
    all_nodes: HashMap<String, CircuitNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircuitNode {
    name: String,
    category: CircuitNodeCategory,
    predecessors: Vec<String>,
    successors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CircuitNodeCategory {
    InputPin,
    OutputPin,
    And,
    Or,
    Not,
}
