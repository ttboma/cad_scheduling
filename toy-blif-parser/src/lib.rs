//! # Introduction
//!
//! **WARNING:** This is a toy parser for the [Berkeley Logic Interchange Format (BLIF)](https://course.ece.cmu.edu/~ee760/760docs/blif.pdf).
//! We only partially implement the BLIF grammar, as shown bellow (the **logic-gate** grammar is our primary care).
//! And we do not fully implement validation of the BLIF grammar, either.
//!
//! As a result,
//! this parser is to parse a BLIF file (but user have to make sure that the BLIF file is valid),
//! and to get the logic-gates information (`.name` command).
//!
//! ```EBNF
//! blif              -> (interface | command)*
//! interface         -> module | inputs | outputs | clock | end
//! command           -> logic_gate | UNRECOGNIZED_LINE
//!
//! model             -> MODEL_PRAGMA decl_model_name
//! decl_model_name   -> IDENTIFIER EOL
//!
//! inputs            -> INPUTS_PRAGMA decl_inputs_list
//! decl_inputs_list  -> (IDENTIFIER)+ EOL
//!
//! outputs           -> OUTPUTS_PRAGMA decl_outputs_list
//! decl_outputs_list -> (IDENTIFIER)+ EOL
//!
//! clock             -> CLOCK_PRAGMA decl_clock_list
//! decl_clock_list   -> (IDENTIFIER)+ EOL
//!
//! end               -> END_PRAGMA EOL
//!
//! logic_gate        -> NAME_PRAGMA decl_output EOL (output EOL)*
//!                    | NAME_PRAGMA decl_inout EOL (input)+ SPACE output EOL
//! ```
//!
//! ## Usage Example
//!
//! [`parser`] is provided to parse a BLIF file.
//!
//! ```
//! use toy_blif_parser::parser;
//!
//! let input = r#"
//!     .model top
//!     .inputs dummy refclk
//!     .outputs clk D5
//!     .names t
//!     1
//!     .gate SB_PLL40_CORE BYPASS=f PLLOUTCORE=clk REFERENCECLK=refclk RESETB=t
//!     .attr src "pll.v:3"
//!     .param DIVF 0000000
//!     .param DIVQ 110
//!     .param DIVR 0000
//!     .param FEEDBACK_PATH "SIMPLE"
//!     .param FILTER_RANGE 001
//!     .param PLLOUT_SELECT "GENCLK_HALF"
//!     .names t D5
//!     1 1
//!     .end
//!     "#;
//! let (s, data) = parser(input).unwrap();
//! ```

use nom::{
    branch::alt,
    bytes::complete::{is_a, is_not, tag, take_till, take_while1},
    character::complete::multispace0,
    combinator::{map, opt},
    error::{context, ParseError, VerboseError},
    multi::{many0, many1},
    sequence::{pair, separated_pair, terminated, tuple},
    IResult,
};

/// Toy parser for BLIF file
pub fn parser(s: &str) -> IResult<&str, Blif, VerboseError<&str>> {
    blif(s)
}

pub mod keyword;
pub mod non_terminal;
pub mod terminal;
use keyword::*;
use non_terminal::*;
use terminal::*;
