/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-22
 */
use std::fmt;
use std::fmt::Formatter;

pub struct PBError {
    message: String,
}

impl PBError {
    pub fn new(message: String) -> PBError {
        PBError { message }
    }
}

impl fmt::Display for PBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for PBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PBError: {}", self.message)
    }
}
