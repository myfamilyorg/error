#![no_std]

extern crate backtrace;
use backtrace::Backtrace;
use core::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct Error {
    code: u128,
    display: fn() -> &'static str,
    bt: Backtrace,
}

impl PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        self.code == other.code
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "ErrorKind={}\n{:?}", (self.display)(), self.bt)
    }
}

impl Error {
    pub const fn new(code: u128, display: fn() -> &'static str, bt: Backtrace) -> Self {
        Self { code, display, bt }
    }
    pub fn code(&self) -> u128 {
        self.code
    }
    pub fn display(&self) -> &'static str {
        (self.display)()
    }
    pub fn set_bt(&mut self, bt: Backtrace) {
        self.bt = bt;
    }
}
