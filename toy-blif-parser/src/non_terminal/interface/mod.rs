use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Interface<'a> {
    Module(Module<'a>),
    Inputs(Inputs<'a>),
    Outputs(Outputs<'a>),
    Clock(Clock<'a>),
    End(End<'a>),
}

pub fn interface(s: &str) -> IResult<&str, Interface, VerboseError<&str>> {
    context(
        "interface",
        alt((
            map(module, Interface::Module),
            map(inputs, Interface::Inputs),
            map(outputs, Interface::Outputs),
            map(clock, Interface::Clock),
            map(end, Interface::End),
        )),
    )(s)
}

pub mod clock;
pub mod end;
pub mod inputs;
pub mod module;
pub mod outputs;
pub use clock::*;
pub use end::*;
pub use inputs::*;
pub use module::*;
pub use outputs::*;
