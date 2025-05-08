#![no_std]

pub mod prelude {
    pub use super::{define_errors_inner, err, errors, Backtrace, Error};
}

extern crate backtrace;
extern crate misc;
pub use backtrace::Backtrace;
pub use misc::simple_hash;

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

#[macro_export]
macro_rules! err {
    ($e:expr) => {{
        let mut e = $e;
        e.set_bt(Backtrace::new());
        Err(e)
    }};
}

#[macro_export]
macro_rules! errors {
    ($($error:ident),*) => {
        use error::simple_hash;
        define_errors_inner!(@count 0, simple_hash(file!(), line!()), $($error),*);
    };
}

#[macro_export]
macro_rules! define_errors_inner {
    (@count $index:expr, $file_hash:expr, $head:ident $(, $tail:ident)*) => {
        #[allow(non_upper_case_globals)]
        pub const $head: Error = Error::new(
            $file_hash + $index,
            || -> &'static str { stringify!($head) },
            Backtrace::init()
        );
        define_errors_inner!(@count $index + 1, $file_hash, $($tail),*);
    };
    (@count $index:expr, $file_hash:expr,) => {};
}
