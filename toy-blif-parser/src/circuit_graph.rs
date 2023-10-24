use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CircuitGraph {
    pub input_nodes: Vec<String>,
    pub gate_nodes: Vec<String>,
    pub nodes: HashMap<String, CircuitNode>,
}

pub struct CircuitGraphBuilder<'a> {
    info: Blif<'a>,
    graph: CircuitGraph,
}

impl<'a> From<Blif<'a>> for CircuitGraph {
    fn from(info: Blif<'a>) -> Self {
        CircuitGraphBuilder {
            info,
            graph: CircuitGraph::default(),
        }
        .push_all_nodes()
        .set_all_predecessors()
        .set_all_successors()
        .set_logic()
        .categorize_nodes()
        .build()
    }
}

impl<'a> CircuitGraphBuilder<'a> {
    fn push_all_nodes(mut self) -> Self {
        for logic_gate in self.info.iter_logic_gates() {
            for &inout in &logic_gate.decl_in_out {
                if !self.graph.nodes.contains_key(inout) {
                    self.graph
                        .nodes
                        .insert(inout.to_owned(), CircuitNode::default());
                }
            }
        }
        self
    }

    fn set_all_predecessors(mut self) -> Self {
        for logic_gate in self.info.iter_logic_gates() {
            let decl_out = *logic_gate.decl_in_out.last().unwrap();
            let out = self.graph.nodes.get_mut(decl_out).unwrap();
            for &input in logic_gate.decl_in_out.iter().rev().skip(1) {
                out.predecessor.push(input.to_owned());
            }
        }
        self
    }

    fn set_all_successors(mut self) -> Self {
        for logic_gate in self.info.iter_logic_gates() {
            let decl_out = *logic_gate.decl_in_out.last().unwrap();
            for &input in logic_gate.decl_in_out.iter().rev().skip(1) {
                self.graph
                    .nodes
                    .get_mut(input)
                    .unwrap()
                    .successor
                    .push(decl_out.to_owned());
            }
        }
        self
    }

    fn set_logic(mut self) -> Self {
        for logic_gate in self.info.iter_logic_gates() {
            let decl_out = *logic_gate.decl_in_out.last().unwrap();
            let out = self.graph.nodes.get_mut(decl_out).unwrap();
            out.pla_logic.add_pla_logic(logic_gate);
        }
        self
    }

    fn categorize_nodes(mut self) -> Self {
        for node in &self.graph.nodes {
            if node.1.predecessor.is_empty() {
                self.graph.input_nodes.push(node.0.clone());
            } else {
                self.graph.gate_nodes.push(node.0.clone());
            }
        }
        self
    }

    fn build(self) -> CircuitGraph {
        self.graph
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CircuitNode {
    pub predecessor: Vec<String>,
    pub successor: Vec<String>,
    pla_logic: PLAGate,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PLAGate {
    pub nor_pins: Vec<AndGate>,
    pub or_pins: Vec<AndGate>,
}

impl CircuitNode {
    pub fn is_and_gate(&self) -> bool {
        self.pla_logic.nor_pins.is_empty()
            && self.pla_logic.or_pins.len() == 1
            && self.pla_logic.or_pins[0].inverted_pins.is_empty()
    }

    pub fn is_or_gate(&self) -> bool {
        self.pla_logic.nor_pins.is_empty()
            && !self.pla_logic.or_pins.is_empty()
            && self
                .pla_logic
                .or_pins
                .iter()
                .all(|g| g.pins.len() == 1 && g.inverted_pins.is_empty())
    }

    pub fn is_not_gate(&self) -> bool {
        (self.pla_logic.nor_pins.is_empty()
            && self.pla_logic.or_pins.len() == 1
            && self.pla_logic.or_pins[0].pins.is_empty()
            && self.pla_logic.or_pins[0].inverted_pins.len() == 1)
            || (self.pla_logic.or_pins.is_empty()
                && self.pla_logic.nor_pins.len() == 1
                && self.pla_logic.nor_pins[0].pins.len() == 1
                && self.pla_logic.nor_pins[0].inverted_pins.is_empty())
    }
}

impl PLAGate {
    pub fn add_pla_logic(&mut self, logic_gate: &LogicGate) {
        for product in &logic_gate.in_out {
            let mut g = AndGate::default();
            for node in product.iter().enumerate().rev().skip(1) {
                match node.1 {
                    Signal::Zero => {
                        g.inverted_pins
                            .push(logic_gate.decl_in_out[node.0].to_owned());
                    }
                    Signal::One => {
                        g.pins.push(logic_gate.decl_in_out[node.0].to_owned());
                    }
                    Signal::DontCare => {}
                }
            }
            match product.last().unwrap() {
                Signal::Zero => {
                    self.nor_pins.push(g);
                }
                Signal::One => {
                    self.or_pins.push(g);
                }
                Signal::DontCare => unreachable!(),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AndGate {
    pub inverted_pins: Vec<String>,
    pub pins: Vec<String>,
}
