use std::fs;
use toy_blif_parser::parser;

#[test]
fn test_sample01() {
    let file_path = "./tests/samples/sample01.blif";
    let contents = fs::read_to_string(file_path).expect("Cannot find sample01.blif");
    let _ = parser(&contents).expect("Some thing wrong with sample01.blif or our parser!!");
}
