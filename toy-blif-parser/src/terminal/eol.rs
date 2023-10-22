use super::*;

pub fn eol(s: &str) -> IResult<&str, (), VerboseError<&str>> {
    let (s, _) = ws(tag("\n"))(s)?;
    Ok((s, ()))
}
