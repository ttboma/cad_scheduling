use crate::*;

pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    parser: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    let concatenated_line = pair(tag("\\\n"), opt(is_a(" \t\r")));
    let comment_line = pair(tag("#"), take_till(|c| c == '\n'));
    terminated(
        parser,
        tuple((
            opt(is_a(" \t\r")),
            many0(concatenated_line),
            opt(comment_line),
        )),
    )
}

pub mod eol;
pub mod identifier;
pub mod pragma;
pub mod signal;
pub mod unrecognized_line;
pub use eol::*;
pub use identifier::*;
pub use pragma::*;
pub use signal::*;
pub use unrecognized_line::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keyword::*;

    #[test]
    fn test_ws() {
        let s = ".names\\\n \t\r\\\n # comment here\n a b c\n";
        let (s, _) = pragma::pragma(NAMES)(s).unwrap();
        assert_eq!(s, "\n a b c\n");
    }
}
