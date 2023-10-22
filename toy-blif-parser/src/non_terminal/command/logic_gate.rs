use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LogicGate<'a> {
    pub pragma: &'a str,
    pub decl_inputs: Vec<&'a str>,
    pub decl_output: &'a str,
    pub inputs: Vec<Signal>,
    pub output: Signal,
}

pub fn logic_gate(s: &str) -> IResult<&str, LogicGate, VerboseError<&str>> {
    context(
        "logic_gate",
        alt((n_input_1_output_logic_gate, constant_logic_gate)),
    )(s)
}

fn n_input_1_output_logic_gate(s: &str) -> IResult<&str, LogicGate, VerboseError<&str>> {
    let (s, pragma) = pragma(NAMES)(s)?;
    let (s, (decl_inputs, decl_output)) =
        map(terminated(many1(identifier), eol), |mut decl_inout| {
            let output = decl_inout.pop().unwrap();
            (decl_inout, output)
        })(s)?;
    let (s, (inputs, output)) = separated_pair(many1(signal), tag(" "), signal)(s)?;
    Ok((
        s,
        LogicGate {
            pragma,
            decl_inputs,
            decl_output,
            inputs,
            output,
        },
    ))
}

fn constant_logic_gate(s: &str) -> IResult<&str, LogicGate, VerboseError<&str>> {
    let (s, pragma) = pragma(NAMES)(s)?;
    let (s, decl_output) = terminated(identifier, eol)(s)?;
    let (s, output) = opt(terminated(signal, eol))(s)?;
    Ok((
        s,
        LogicGate {
            pragma,
            decl_inputs: vec![],
            decl_output,
            inputs: vec![],
            output: output.unwrap_or(Signal::Zero),
        },
    ))
}
