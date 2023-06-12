use std::io::BufRead;
use std::panic;

use crate::io;


// With regular `catch_unwind` the test succeeds, but the panic is still printed. This is confusing
// to humans and to machines alike (VSCode highlights the panic location as if it were an error).
// Copied from https://stackoverflow.com/a/59211519/3092679.
pub fn catch_unwind_silent<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) -> std::thread::Result<R> {
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(f);
    panic::set_hook(prev_hook);
    result
}

pub fn reader_from_string(input: impl ToString) -> io::Reader<impl BufRead> {
    io::Reader::new(std::io::Cursor::new(input.to_string().into_bytes()))
}
