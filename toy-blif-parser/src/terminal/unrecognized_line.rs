use super::*;

pub fn unrecognized_line(s: &str) -> IResult<&str, (), VerboseError<&str>> {
    let (s, _) = is_not("\n")(s)?;
    let (s, _) = eol(s)?;
    Ok((s, ()))
}
