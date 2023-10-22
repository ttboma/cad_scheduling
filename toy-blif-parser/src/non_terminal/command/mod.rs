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
