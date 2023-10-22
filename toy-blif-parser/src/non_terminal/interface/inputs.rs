use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Inputs<'a> {
    pub pragma: &'a str,
    pub node: Vec<&'a str>,
}

pub fn inputs(s: &str) -> IResult<&str, Inputs, VerboseError<&str>> {
    context("inputs", inputs_derive)(s)
}

fn inputs_derive(s: &str) -> IResult<&str, Inputs, VerboseError<&str>> {
    let (s, pragma) = pragma(INPUTS)(s)?;
    let (s, node) = many1(identifier)(s)?;
    let (s, _) = eol(s)?;
    Ok((s, Inputs { pragma, node }))
}
