use constrained_scheduling::MlRcs;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use toy_blif_parser::*;

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args_os().collect();

    if args[1] == "-l" {
        run_ml_rcs(&args)
    } else {
        Ok(())
    }
}

fn run_ml_rcs(args: &[OsString]) -> io::Result<()> {
    let contents =
        fs::read_to_string(&args[2]).unwrap_or_else(|_| panic!("Cannot find {:?}", &args[2]));
    let (_, data) = parser(&contents).expect("Some thing wrong with the blif file or our parser!!");
    let graph = CircuitGraph::from(data);
    let and_constraint = args[3].clone().into_string().unwrap().parse().unwrap();
    let or_constraint = args[4].clone().into_string().unwrap().parse().unwrap();
    let not_constraint = args[5].clone().into_string().unwrap().parse().unwrap();
    let result = graph.ml_rcs(and_constraint, or_constraint, not_constraint);

    println!("Resource-constrained scheduling");
    for i in 0..result.and.len() {
        println!(
            "{} : {:?} {:?} {:?}",
            i, result.and[i], result.or[i], result.not[i]
        )
    }
    println!("#AND: {}", and_constraint);
    println!("#OR: {}", or_constraint);
    println!("#NOT: {}", not_constraint);
    println!("#END");
    Ok(())
}
