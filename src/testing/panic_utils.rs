use std::panic;


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

#[macro_export]
macro_rules! assert_panics {
    ($e:expr) => {
        assert!($crate::testing::panic_utils::catch_unwind_silent($e).is_err());
    };
}
