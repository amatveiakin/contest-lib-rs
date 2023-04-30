use std::cell::RefCell;
use std::{fmt, io, mem, str};


// TODO: Store StdinLock
// TODO: Optimize using `io::BufRead::{fill_buf, consume}`
#[derive(Default)]
struct InputBuffer {
    line: Vec<u8>,
    pos: usize,
}

thread_local! {
    static CURRENT_LINE: RefCell<InputBuffer> = RefCell::new(InputBuffer::default());
}

impl InputBuffer {
    fn has_data(&self) -> bool {
        self.pos < self.line.len()
    }

    fn get_next_line(&mut self) {
        // Reuse allocated memory from the previous line: `clear` sets length to zero, but keeps
        // capacity. These bytes are later reused to store the next line. Since the length is zero,
        // `String::from_utf8` trivially succeeds in O(1).
        self.line.clear();
        let mut s = String::from_utf8(mem::take(&mut self.line)).unwrap();
        io::stdin().read_line(&mut s).unwrap();
        self.line = s.into_bytes();
        self.pos = 0;
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.line.len() && self.line[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    fn consume_word(&mut self) -> &[u8] {
        let start = self.pos;
        while self.pos < self.line.len() && !self.line[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
        &self.line[start..self.pos]
    }

    fn consume_till_eol(&mut self) -> &[u8] {
        let start = self.pos;
        self.pos = self.line.len();
        &self.line[start..]
    }
}

pub fn read<T>() -> T
where
    T: str::FromStr,
    <T as str::FromStr>::Err: fmt::Debug,
{
    CURRENT_LINE.with(|buf| {
        let mut buf = buf.borrow_mut();
        buf.skip_whitespace();
        while !buf.has_data() {
            buf.get_next_line();
            buf.skip_whitespace();
        }
        let word = str::from_utf8(buf.consume_word()).unwrap();
        word.parse().unwrap()
    })
}

pub fn read_i32() -> i32 { read() }
pub fn read_u32() -> u32 { read() }
pub fn read_i64() -> i64 { read() }
pub fn read_u64() -> u64 { read() }

pub fn read_word() -> String { read() }

pub fn read_line() -> String {
    // TODO: Comment on empty lines and interactions with other `read`s
    CURRENT_LINE.with(|buf| {
        let mut buf = buf.borrow_mut();
        let line = str::from_utf8(buf.consume_till_eol()).unwrap().to_owned();
        buf.get_next_line();
        line
    })
}

pub fn read_vec<T>(len: usize) -> Vec<T>
where
    T: str::FromStr,
    <T as str::FromStr>::Err: fmt::Debug,
{
    (0..len).map(|_| read::<T>()).collect()
}

pub fn read_vec_i32(len: usize) -> Vec<i32> { read_vec(len) }
pub fn read_vec_u32(len: usize) -> Vec<u32> { read_vec(len) }
pub fn read_vec_i64(len: usize) -> Vec<i64> { read_vec(len) }
pub fn read_vec_u64(len: usize) -> Vec<u64> { read_vec(len) }

pub trait Emittable {
    fn emit(&self);
}

macro_rules! trait_for_value_and_ref {
    ( impl<{ $( $cond:tt )* }> $trait_name:ident for $t:ty { $( $body:tt )* } ) => {
        impl<    $( $cond )*> $trait_name for         $t { $( $body )* }
        impl<'a, $( $cond )*> $trait_name for &'a     $t { $( $body )* }
        impl<'a, $( $cond )*> $trait_name for &'a mut $t { $( $body )* }
    }
}

macro_rules! simple_emittable {
    ( $( $t:ty ),* ) => {
        $(
            trait_for_value_and_ref!(impl<{}> Emittable for $t {
                fn emit(&self) { print!("{} ", self); }
            });
        )*
    }
}
simple_emittable!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f64, f32, String);

trait_for_value_and_ref!(impl<{T: Emittable}> Emittable for Vec<T> {
    fn emit(&self) { self.iter().for_each(|v| v.emit()); }
});

pub fn emit_impl<T: Emittable>(value: T) { value.emit(); }

#[macro_export]
macro_rules! emit {
    ( $( $value:expr ),* ) => {{
        $( crate::io::emit_impl($value); )*
    }};
}

#[macro_export]
macro_rules! emitln {
    ( $($value:expr),* ) => {{
        emit!( $($value),* );
        println!();
    }};
}
