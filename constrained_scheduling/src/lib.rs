use std::{
    collections::{BTreeSet, HashMap},
    fmt::Debug,
};
use toy_blif_parser::CircuitGraph;

pub trait MlRcs {
    fn ml_rcs(
        &self,
        and_constraint: usize,
        or_constraint: usize,
        not_constraint: usize,
    ) -> MlRcsScheduling;
}

#[derive(Debug, Default)]
pub struct MlRcsScheduling<'a> {
    pub and: Vec<Vec<&'a str>>,
    pub or: Vec<Vec<&'a str>>,
    pub not: Vec<Vec<&'a str>>,
}

impl MlRcs for CircuitGraph {
    fn ml_rcs(
        &self,
        and_constraint: usize,
        or_constraint: usize,
        not_constraint: usize,
    ) -> MlRcsScheduling {
        let mut and_ready = BTreeSet::<&str>::new();
        let mut or_ready = BTreeSet::<&str>::new();
        let mut not_ready = BTreeSet::<&str>::new();
        let mut waiting_number = HashMap::<&str, usize>::new();

        for handle in &self.gate_nodes {
            waiting_number.insert(handle.as_str(), 0);
        }
        for handle in &self.gate_nodes {
            let node = &self.nodes[handle];
            for predecessor_handle in node.predecessor.iter() {
                if !self.nodes[predecessor_handle].predecessor.is_empty() {
                    // predecessor are not input nodes
                    *waiting_number.get_mut(handle.as_str()).unwrap() += 1;
                }
            }

            if waiting_number[handle.as_str()] != 0 {
                continue;
            }
            if node.is_and_gate() {
                and_ready.insert(handle.as_str());
            } else if node.is_or_gate() {
                or_ready.insert(handle.as_str());
            } else {
                not_ready.insert(handle.as_str());
            }
        }

        let mut scheduling = MlRcsScheduling::default();

        while !and_ready.is_empty() || !or_ready.is_empty() || !not_ready.is_empty() {
            let mut scheduling_and = vec![];
            let mut scheduling_or = vec![];
            let mut scheduling_not = vec![];

            let mut ready_successor = vec![];
            for _ in 0..and_constraint {
                if let Some(v) = and_ready.pop_last() {
                    scheduling_and.push(v);
                    for successor_handle in &self.nodes[v].successor {
                        *waiting_number.get_mut(successor_handle.as_str()).unwrap() -= 1;
                        if waiting_number[successor_handle.as_str()] == 0 {
                            ready_successor.push(successor_handle.as_str());
                        }
                    }
                } else {
                    break;
                }
            }
            for _ in 0..or_constraint {
                if let Some(v) = or_ready.pop_last() {
                    scheduling_or.push(v);
                    for successor_handle in &self.nodes[v].successor {
                        *waiting_number.get_mut(successor_handle.as_str()).unwrap() -= 1;
                        if waiting_number[successor_handle.as_str()] == 0 {
                            ready_successor.push(successor_handle.as_str());
                        }
                    }
                } else {
                    break;
                }
            }
            for _ in 0..not_constraint {
                if let Some(v) = not_ready.pop_last() {
                    scheduling_not.push(v);
                    for successor_handle in &self.nodes[v].successor {
                        *waiting_number.get_mut(successor_handle.as_str()).unwrap() -= 1;
                        if waiting_number[successor_handle.as_str()] == 0 {
                            ready_successor.push(successor_handle.as_str());
                        }
                    }
                } else {
                    break;
                }
            }
            for handle in ready_successor {
                let node = &self.nodes[handle];
                if node.is_and_gate() {
                    and_ready.insert(handle);
                } else if node.is_or_gate() {
                    or_ready.insert(handle);
                } else {
                    not_ready.insert(handle);
                }
            }
            scheduling.and.push(scheduling_and);
            scheduling.or.push(scheduling_or);
            scheduling.not.push(scheduling_not);
        }
        scheduling
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use toy_blif_parser::parser;

    #[test]
    fn test_ml_rcs() {
        let input_blif = r#"
            .names a d g
            1- 1
            -1 1
            .names a c h
            11 1
            .names c i
            0 1
            .names d e f j
            1-- 1
            -1- 1
            --1 1
            .names g h i k
            1-- 1
            -1- 1
            --1 1
            .names h i j l
            111 1
            .names i j m
            11 1
            .names l m n
            11 1
            .names b h k o
            111 1
            .names g p
            0 1
            .names n q
            0 1
        "#;
        let (_, data) = parser(&input_blif).unwrap();
        let _ = CircuitGraph::from(data);
    }
}
