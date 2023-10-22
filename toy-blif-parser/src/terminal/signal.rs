use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Signal {
    One,
    Zero,
    DontCare,
}

pub fn signal(s: &str) -> IResult<&str, Signal, VerboseError<&str>> {
    context("signal", signal_impl)(s)
}

fn signal_impl(s: &str) -> IResult<&str, Signal, VerboseError<&str>> {
    let (s, a) = alt((tag("1"), tag("0"), tag("-")))(s)?;
    match a {
        "1" => Ok((s, Signal::One)),
        "0" => Ok((s, Signal::Zero)),
        "-" => Ok((s, Signal::DontCare)),
        _ => unreachable!(),
    }
}
