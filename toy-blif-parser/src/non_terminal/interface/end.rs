use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct End<'a> {
    pub pragma: &'a str,
}

pub fn end(s: &str) -> IResult<&str, End, VerboseError<&str>> {
    context("end", end_derive)(s)
}

fn end_derive(s: &str) -> IResult<&str, End, VerboseError<&str>> {
    let (s, pragma) = pragma(END)(s)?;
    let (s, _) = eol(s)?;
    Ok((s, End { pragma }))
}
