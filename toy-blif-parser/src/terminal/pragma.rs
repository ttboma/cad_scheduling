use super::*;

pub fn pragma<'a, 'b: 'a>(
    pragma: &'b str,
) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, VerboseError<&'a str>> {
    context("pragma", ws(pragma_impl(pragma)))
}

fn pragma_impl<'a, 'b: 'a>(
    pragma: &'b str,
) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, VerboseError<&'a str>> {
    move |s| {
        let (s, pragma) = tag(pragma)(s)?;
        Ok((s, pragma))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keyword::*;

    #[test]
    fn test_pragma() {
        let s = ".names a b c\n";
        let (s, pragma) = pragma(NAMES)(s).unwrap();
        assert_eq!(s, "a b c\n");
        assert_eq!(pragma, ".names");
    }
}
