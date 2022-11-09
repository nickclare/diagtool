use std::io::Read;

use crate::model::DiagRef;
use crate::{Error, Result};

pub(crate) struct Parser;

impl Parser {
    pub fn parse<R: Read>(input: R) -> Result<DiagRef> {
        todo!()
    }
}
