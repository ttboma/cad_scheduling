use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Outputs<'a> {
    pub pragma: &'a str,
    pub node: Vec<&'a str>,
}

pub fn outputs(s: &str) -> IResult<&str, Outputs, VerboseError<&str>> {
    context("outputs", outputs_derive)(s)
}

fn outputs_derive(s: &str) -> IResult<&str, Outputs, VerboseError<&str>> {
    let (s, pragma) = pragma(OUTPUTS)(s)?;
    let (s, node) = many1(identifier)(s)?;
    let (s, _) = eol(s)?;
    Ok((s, Outputs { pragma, node }))
}
