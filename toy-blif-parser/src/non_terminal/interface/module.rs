use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Module<'a> {
    pub pragma: &'a str,
    pub name: &'a str,
}

pub fn module(s: &str) -> IResult<&str, Module, VerboseError<&str>> {
    context("module", module_derive)(s)
}

fn module_derive(s: &str) -> IResult<&str, Module, VerboseError<&str>> {
    let (s, pragma) = pragma(MODEL)(s)?;
    let (s, name) = identifier(s)?;
    let (s, _) = eol(s)?;
    Ok((s, Module { pragma, name }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module() {
        let s = ".model test_module\n";
        let (_, module) = module(s).unwrap();
        assert_eq!(module.name, "test_module");
    }
}
