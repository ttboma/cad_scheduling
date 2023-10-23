use std::fs;
use toy_blif_parser::{parser, CircuitGraph};

#[test]
fn test_sample01() {
    let file_path = "./tests/samples/sample01.blif";
    let contents = fs::read_to_string(file_path).expect("Cannot find sample01.blif");
    let (_, _) = parser(&contents).expect("Some thing wrong with sample01.blif or our parser!!");
}

#[test]
fn test_sample02() {
    let file_path = "./tests/samples/sample02.blif";
    let contents = fs::read_to_string(file_path).expect("Cannot find sample02.blif");
    let (_, data) = parser(&contents).expect("Some thing wrong with sample02.blif or our parser!!");
    let graph = CircuitGraph::from(data);
    assert!(graph.nodes["g"].pla_logic.is_or_gate());
    assert!(graph.nodes["h"].pla_logic.is_and_gate());
    assert!(graph.nodes["i"].pla_logic.is_not_gate());
    assert!(graph.nodes["j"].pla_logic.is_or_gate());
    assert!(graph.nodes["k"].pla_logic.is_or_gate());
    assert!(graph.nodes["l"].pla_logic.is_and_gate());
    assert!(graph.nodes["m"].pla_logic.is_and_gate());
    assert!(graph.nodes["n"].pla_logic.is_and_gate());
    assert!(graph.nodes["o"].pla_logic.is_and_gate());
    assert!(graph.nodes["p"].pla_logic.is_not_gate());
    assert!(graph.nodes["q"].pla_logic.is_not_gate());
}
