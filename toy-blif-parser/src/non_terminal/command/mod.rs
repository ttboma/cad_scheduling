use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Command<'a> {
    LogicGate(LogicGate<'a>),
    UnrecognizedLine,
}

pub fn command(s: &str) -> IResult<&str, Command, VerboseError<&str>> {
    context(
        "command",
        alt((
            map(logic_gate, Command::LogicGate),
            map(unrecognized_line, |_| Command::UnrecognizedLine),
        )),
    )(s)
}

pub mod logic_gate;
pub use logic_gate::*;

#[cfg(test)]
mod tests {
    use nom::sequence::preceded;

    use super::*;

    #[test]
    fn test_commands() {
        let input = r#"
            .names b c d f
            101 1
            .names j
            .names k
            1
            .names b c d g
            11- 1
            --1 1
        "#;
        let (_, data) = preceded(multispace0, many1(command))(input).unwrap();
        assert_eq!(data.len(), 4);
    }
}
