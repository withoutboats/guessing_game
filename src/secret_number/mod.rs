use std::fmt;
use std::fmt::Display;

#[cfg(test)]
mod tests;

pub struct Secret {
}

impl Secret {
    pub fn random() -> Secret {
        unimplemented!()
    }

    fn new(secret: u16) -> Secret {
        unimplemented!()
    }

    pub fn guess(&self, guess: &str) -> Result<u32, GuessError> {
        unimplemented!()
    }

    fn secret(&self) -> u16 {
        unimplemented!()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum GuessError {
    TooHigh(u16),
    TooLow(u16),
    NotANumber,
}

impl Display for GuessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        unimplemented!()
    }
}
