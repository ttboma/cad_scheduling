use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Clock<'a> {
    pub pragma: &'a str,
    pub node: Vec<&'a str>,
}

pub fn clock(s: &str) -> IResult<&str, Clock, VerboseError<&str>> {
    context("clock", inputs_derive)(s)
}

fn inputs_derive(s: &str) -> IResult<&str, Clock, VerboseError<&str>> {
    let (s, pragma) = pragma(CLOCK)(s)?;
    let (s, node) = many1(identifier)(s)?;
    let (s, _) = eol(s)?;
    Ok((s, Clock { pragma, node }))
}
