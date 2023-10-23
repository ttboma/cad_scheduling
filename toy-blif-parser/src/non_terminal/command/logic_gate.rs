use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LogicGate<'a> {
    pub pragma: &'a str,
    pub decl_in_out: Vec<&'a str>,
    pub in_out: Vec<Vec<Signal>>,
}

pub fn logic_gate(s: &str) -> IResult<&str, LogicGate, VerboseError<&str>> {
    context(
        "logic_gate",
        alt((n_input_1_output_logic_gate, constant_logic_gate)),
    )(s)
}

fn n_input_1_output_logic_gate(s: &str) -> IResult<&str, LogicGate, VerboseError<&str>> {
    let (s, pragma) = pragma(NAMES)(s)?;
    let (s, decl_in_out) = terminated(many1(identifier), eol)(s)?;
    let (s, in_out) = map(
        many1(terminated(
            separated_pair(many1(signal), tag(" "), signal),
            eol,
        )),
        |in_out| {
            in_out
                .into_iter()
                .map(|(mut a, b)| {
                    a.push(b);
                    a
                })
                .collect()
        },
    )(s)?;
    Ok((
        s,
        LogicGate {
            pragma,
            decl_in_out,
            in_out,
        },
    ))
}

fn constant_logic_gate(s: &str) -> IResult<&str, LogicGate, VerboseError<&str>> {
    let (s, pragma) = pragma(NAMES)(s)?;
    let (s, decl_out) = terminated(identifier, eol)(s)?;
    let (s, output) = opt(terminated(signal, eol))(s)?;
    Ok((
        s,
        LogicGate {
            pragma,
            decl_in_out: vec![decl_out],
            in_out: vec![vec![output.unwrap_or(Signal::Zero)]],
        },
    ))
}
