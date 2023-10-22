use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Blif<'a> {
    pub interface: Vec<Interface<'a>>,
    pub commands: Vec<Command<'a>>,
}

impl<'a> From<Vec<Format<'a>>> for Blif<'a> {
    fn from(value: Vec<Format<'a>>) -> Self {
        let mut interface: Vec<Interface<'a>> = Vec::new();
        let mut commands: Vec<Command<'a>> = Vec::new();
        value.into_iter().for_each(|f| match f {
            Format::Command(c) => commands.push(c),
            Format::Interface(i) => interface.push(i),
        });
        Self {
            interface,
            commands,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Format<'a> {
    Command(Command<'a>),
    Interface(Interface<'a>),
}

pub fn blif(s: &str) -> IResult<&str, Blif, VerboseError<&str>> {
    let (s, _) = multispace0(s)?;
    let (s, value) = many1(alt((
        map(interface, Format::Interface),
        map(command, Format::Command),
    )))(s)?;

    Ok((s, value.into()))
}

pub mod command;
pub mod interface;
pub use command::*;
pub use interface::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blif() {
        let input = r#"
            .model top
            .inputs dummy refclk
            .outputs clk D5
            .names t
            1
            .gate SB_PLL40_CORE BYPASS=f PLLOUTCORE=clk REFERENCECLK=refclk RESETB=t
            .attr src "pll.v:3"
            .param DIVF 0000000
            .param DIVQ 110
            .param DIVR 0000
            .param FEEDBACK_PATH "SIMPLE"
            .param FILTER_RANGE 001
            .param PLLOUT_SELECT "GENCLK_HALF"
            .names t D5
            1 1
            .end
            "#;
        let (s, data) = blif(input).unwrap();
        println!("{:#?}", s);
        println!("{:#?}", data);
    }
}
