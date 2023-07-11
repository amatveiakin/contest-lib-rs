use std::{fmt, mem, str};

pub struct Reader<R: std::io::BufRead> {
    reader: R,
    buf: InputBuffer,
}

impl<R: std::io::BufRead> Reader<R> {
    pub fn new(reader: R) -> Self {
        Reader {
            reader,
            buf: Default::default(),
        }
    }

    pub fn atom<T>(&mut self) -> T
    where
        T: str::FromStr,
        <T as str::FromStr>::Err: fmt::Debug,
    {
        self.buf.skip_whitespace();
        while !self.buf.has_data() {
            self.get_next_line();
            self.buf.skip_whitespace();
        }
        let word = str::from_utf8(self.buf.consume_word()).unwrap();
        word.parse().unwrap()
    }

    pub fn i32(&mut self) -> i32 { self.atom() }
    pub fn u32(&mut self) -> u32 { self.atom() }
    pub fn i64(&mut self) -> i64 { self.atom() }
    pub fn u64(&mut self) -> u64 { self.atom() }
    pub fn usize(&mut self) -> usize { self.atom() }
    pub fn f32(&mut self) -> f32 { self.atom() }
    pub fn f64(&mut self) -> f64 { self.atom() }
    pub fn word(&mut self) -> String { self.atom() }

    pub fn atoms<T, const N: usize>(&mut self) -> [T; N]
    where
        T: str::FromStr,
        <T as str::FromStr>::Err: fmt::Debug,
    {
        [(); N].map(|()| self.atom())
    }

    pub fn i32s<const N: usize>(&mut self) -> [i32; N] { self.atoms() }
    pub fn u32s<const N: usize>(&mut self) -> [u32; N] { self.atoms() }
    pub fn i64s<const N: usize>(&mut self) -> [i64; N] { self.atoms() }
    pub fn u64s<const N: usize>(&mut self) -> [u64; N] { self.atoms() }
    pub fn usizes<const N: usize>(&mut self) -> [usize; N] { self.atoms() }
    pub fn f32s<const N: usize>(&mut self) -> [f32; N] { self.atoms() }
    pub fn f64s<const N: usize>(&mut self) -> [f64; N] { self.atoms() }
    pub fn words<const N: usize>(&mut self) -> [String; N] { self.atoms() }

    // Reads the whole inline (including whitespace) until '\n'. If other `read`s were used earlier,
    // reads the rest of the previous line first, even if it's empty. You probably want to skip
    // first `read_line` result.
    pub fn line(&mut self) -> String {
        if !self.buf.line_in_progress {
            self.get_next_line();
        }
        let line = str::from_utf8(self.buf.consume_till_eol()).unwrap();
        let line = line.trim_end_matches(['\n', '\r']);
        let line = line.to_owned();
        line
    }

    pub fn vec<T>(&mut self, len: usize) -> Vec<T>
    where
        T: str::FromStr,
        <T as str::FromStr>::Err: fmt::Debug,
    {
        (0..len).map(|_| self.atom::<T>()).collect()
    }

    pub fn vec_i32(&mut self, len: usize) -> Vec<i32> { self.vec(len) }
    pub fn vec_u32(&mut self, len: usize) -> Vec<u32> { self.vec(len) }
    pub fn vec_i64(&mut self, len: usize) -> Vec<i64> { self.vec(len) }
    pub fn vec_u64(&mut self, len: usize) -> Vec<u64> { self.vec(len) }
    pub fn vec_usize(&mut self, len: usize) -> Vec<usize> { self.vec(len) }
    pub fn vec_f32(&mut self, len: usize) -> Vec<f32> { self.vec(len) }
    pub fn vec_f64(&mut self, len: usize) -> Vec<f64> { self.vec(len) }
    pub fn vec_word(&mut self, len: usize) -> Vec<String> { self.vec(len) }

    fn get_next_line(&mut self) {
        // Reuse allocated memory from the previous line: `clear` sets length to zero, but keeps
        // capacity. These bytes are later reused to store the next line. Since the length is zero,
        // `String::from_utf8` trivially succeeds in O(1).
        self.buf.line.clear();
        let mut s = String::from_utf8(mem::take(&mut self.buf.line)).unwrap();
        self.reader.read_line(&mut s).unwrap();
        self.buf.line = s.into_bytes();
        self.buf.pos = 0;
        self.buf.line_in_progress = true;
    }
}

pub trait Emittable {
    fn emit(&self, writer: &mut impl std::io::Write);
}

#[macro_export]
macro_rules! trait_for_value_and_ref {
    ( impl<{ $( $cond:tt )* }> $trait_name:ident for $t:ty { $( $body:tt )* } ) => {
        impl<    $( $cond )*> $trait_name for         $t { $( $body )* }
        impl<'a, $( $cond )*> $trait_name for &'a     $t { $( $body )* }
        impl<'a, $( $cond )*> $trait_name for &'a mut $t { $( $body )* }
    }
}

#[macro_export]
macro_rules! simple_emittable {
    ( $( $t:ty ),* $(,)? ) => {
        $(
            trait_for_value_and_ref!(impl<{}> Emittable for $t {
                fn emit(&self, writer: &mut impl std::io::Write) {
                    write!(writer, "{} ", self).unwrap();
                }
            });
        )*
    }
}

// TODO: Try to replace `simple_emittable` with `impl<T: Display> Emittable for T`
//   (or at least, `String` and `str` with `ToString<T>`).
// Problem:
// "
//   conflicting implementations of trait `io::Emittable` for type `std::vec::Vec<_>`
//   upstream crates may add a new impl of trait `std::fmt::Display` for type `std::vec::Vec<_>`
//   in future versions
// "
simple_emittable!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f64, f32,
    String, str, char
);

trait_for_value_and_ref!(impl<{T: Emittable}> Emittable for Vec<T> {
    fn emit(&self, writer: &mut impl std::io::Write) {
        self.iter().for_each(|v| v.emit(writer));
    }
});

pub fn emit_impl<T: Emittable>(writer: &mut impl std::io::Write, value: T) {
    value.emit(writer);
}

// TODO: Make the macro imply `&`, like `write!` does.
#[macro_export]
macro_rules! emit {
    ( $dst:expr, $( $value:expr ),* ) => {{
        $( $crate::io::emit_impl($dst, $value); )*
    }};
}

#[macro_export]
macro_rules! emitln {
    ( $dst:expr, $($value:expr),* ) => {{
        $crate::emit!($dst, $($value),*);
        writeln!($dst).unwrap();
    }};
}

// TODO: Replace with using `std::io::BufRead::{fill_buf, consume}`
//   (see https://codeforces.com/contest/1151/submission/53175781)
#[derive(Default)]
struct InputBuffer {
    line: Vec<u8>,
    pos: usize,
    line_in_progress: bool, // line is considered in progress until `consume_till_eol` is called
}

impl InputBuffer {
    fn has_data(&self) -> bool {
        self.pos < self.line.len()
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
        self.line_in_progress = false;
        &self.line[start..]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_single() {
        let input = "hi 1 3 2.25 1e4\nbye\n";
        let mut read = Reader::new(std::io::Cursor::new(input.to_string().into_bytes()));
        assert_eq!(read.word(), "hi");
        assert_eq!(read.i32(), 1);
        assert_eq!(read.u32(), 3);
        assert_eq!(read.f64(), 2.25);
        assert_eq!(read.f64(), 1e4);
        assert_eq!(read.word(), "bye");
    }

    #[test]
    fn read_multi() {
        let input = "hello world 1 2 3 7.77";
        let mut read = Reader::new(std::io::Cursor::new(input.to_string().into_bytes()));
        let [x, y] = read.words();
        assert_eq!(x, "hello");
        assert_eq!(y, "world");
        let [a, b, c] = read.i32s();
        assert_eq!(a, 1);
        assert_eq!(b, 2);
        assert_eq!(c, 3);
        let [f] = read.f32s();
        assert_eq!(f, 7.77);
    }

    #[test]
    fn read_vec() {
        let input = "4\nwhoa so many words\n4 2 4 5";
        let mut read = Reader::new(std::io::Cursor::new(input.to_string().into_bytes()));
        let n = read.usize();
        let w = read.vec_word(n);
        let l = read.vec_usize(n);
        assert_eq!(w, vec!["whoa", "so", "many", "words"]);
        assert_eq!(l, vec![4, 2, 4, 5]);
    }
}
