use ab_glyph::FontRef;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::term::fbterm::FBTerm;

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ({
        write!($crate::print::TERM.lock(), "{}", format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! kprintln {
    () => (write!($crate::print::TERM.lock(), "\n"));
    ($($arg:tt)*) => (write!($crate::print::TERM.lock(), "{}\n", format_args!($($arg)*)));
}

lazy_static! {
    pub static ref TERM: Mutex<FBTerm<'static>> = Mutex::new(FBTerm::new_unset(
        FontRef::try_from_slice(include_bytes!("term/font.ttf")).unwrap()
    ));
}
