//! patch-rs is a parser library for [Unified Format]
//! (https://www.gnu.org/software/diffutils/manual/html_node/Unified-Format.html#Unified-Format)
//! diffs.
//!
//! GVR also honed down the spec a bit more:
//! http://www.artima.com/weblogs/viewpost.jsp?thread=164293

extern crate chrono;
extern crate nom;

use nom::IResult;
use std::error::Error;

use self::parser::patch;
pub use self::parser::{File, FileMetadata, Hunk, Line, Patch, Range};

mod parser;

#[derive(Debug)]
pub enum PatchError {
    ParseError,
}

impl std::fmt::Display for PatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            PatchError::ParseError => write!(f, "Error while parsing"),
        }
    }
}

impl Error for PatchError {
    fn description(&self) -> &str {
        match *self {
            PatchError::ParseError => "parse error",
        }
    }
}

pub fn parse(diff: &str) -> Result<Patch, PatchError> {
    match patch(diff.as_bytes()) {
        IResult::Done(_, p) => Ok(p),
        IResult::Incomplete(_) | IResult::Error(_) => Err(PatchError::ParseError),
    }
}
