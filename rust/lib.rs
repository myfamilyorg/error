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

    pub unsafe fn bt_as_ptr(&self) -> *const u8 {
        self.bt.as_ptr()
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
        #[cfg(not(test))]
        use error::simple_hash;
        #[cfg(test)]
        use self::error::simple_hash;
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

// Test framework requires Debug
#[cfg(test)]
impl core::fmt::Debug for Error {
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate as error;

    errors!(TestErr1, TestErr2);

    #[test]
    fn test_err1() {
        let mut bt = Backtrace::init();
        bt.capture();
        let e = Error::new(1, || -> &'static str { "test" }, bt);
        assert_eq!(e.code(), 1);
    }

    #[test]
    fn test_err2() {
        let e1: Result<(), _> = err!(TestErr1);
        let e2: Result<(), _> = err!(TestErr2);
        let e3: Result<(), _> = err!(TestErr2);

        assert_ne!(e1.unwrap_err(), e2.clone().unwrap_err());
        assert_eq!(e3.unwrap_err(), e2.unwrap_err());
    }
}
