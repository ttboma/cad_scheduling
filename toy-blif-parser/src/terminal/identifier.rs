use super::*;

pub fn identifier(s: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context("identifier", ws(identifier_impl))(s)
}

fn identifier_impl(s: &str) -> IResult<&str, &str, VerboseError<&str>> {
    let (s, name) = take_while1(|c: char| c.is_alphanumeric() || c == '_')(s)?;
    Ok((s, name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decl_model_name() {
        let s = "a \n";
        let (s, name) = identifier(s).unwrap();
        assert_eq!(s, "\n");
        assert_eq!(name, "a");
    }
}
