#![no_std]

extern crate backtrace;

use backtrace::Backtrace;

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

pub fn real_main(_argc: i32, _argv: *const *const i8) -> i32 {
    backtrace::backtrace_fn()
}
