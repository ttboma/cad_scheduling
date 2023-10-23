use std::env;
use std::fs;
use std::io;
use std::io::Write;
use toy_blif_parser::*;

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args_os().collect();

    if args.len() != 2 {
        panic!("usage: blif_parser <blif-file-name>");
    }

    let contents =
        fs::read_to_string(&args[1]).unwrap_or_else(|_| panic!("Cannot find {:?}", &args[1]));
    let (_, data) = parser(&contents).expect("Some thing wrong with the blif file or our parser!!");
    let graph = CircuitGraph::from(data);

    loop {
        print!("Please input a node: ");
        io::stdout().flush().unwrap();

        let mut v = String::new();
        io::stdin().read_line(&mut v)?;
        let v = v.trim();

        if v == "0" {
            break Ok(());
        }

        match graph.nodes.get(v) {
            Some(node) => {
                println!("predecessor: {:?}", &node.predecessor);
                println!("successor: {:?}", &node.successor);
            }
            None => {
                println!("node {} does not exist", &v);
            }
        }
    }
}
