// A 2D array. Just an array, with no matrix operations.

use std::io::BufRead;
use std::{fmt, mem, ops, str};

use crate::io::Reader;
use crate::iterutils::Iterutils;
use crate::relax::RelaxMinMax;


#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Array2D<T> {
    // Invariant: data.len() == shape.0 * shape.1
    data: Vec<T>,
    shape: (usize, usize),
}

impl<T> Array2D<T> {
    pub fn new() -> Self {
        Self { data: Vec::new(), shape: (0, 0) }
    }

    pub fn with_shape(n_rows: usize, n_cols: usize) -> Self
    where
        T: Default,
    {
        // Note: `vec![T::default(); len]` is not used because it requires `T: Clone`.
        let len = n_rows * n_cols;
        let mut data = Vec::with_capacity(len);
        for _ in 0..len {
            data.push(T::default());
        }
        Self { data, shape: (n_rows, n_cols) }
    }

    pub fn num_rows(&self) -> usize { self.shape.0 }
    pub fn num_cols(&self) -> usize { self.shape.1 }
    pub fn shape(&self) -> (usize, usize) { self.shape }

    // Requires the array be nonempty.
    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        let (_, n_cols) = self.shape;
        self.data.chunks(n_cols)
    }

    pub fn iter_enumerated(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        let (_, n_cols) = self.shape;
        self.data.iter().enumerate().map(move |(i, x)| {
            ((i / n_cols, i % n_cols), x)
        })
    }

    pub fn map<U>(self, f: impl Fn(T) -> U) -> Array2D<U> {
        let data = self.data.into_iter().map(|x| f(x)).collect();
        Array2D { data, shape: self.shape }
    }
    pub fn map_enumerated<U>(self, f: impl Fn((usize, usize), T) -> U) -> Array2D<U> {
        let (_, n_cols) = self.shape;
        let data = self.data.into_iter().enumerate().map(|(i, x)| f((i / n_cols, i % n_cols), x)).collect();
        Array2D { data, shape: self.shape }
    }

    pub fn transpose(mut self) -> Self
    where
        T: Default,
    {
        let (n, m) = self.shape;
        let mut data = Vec::with_capacity(n * m);
        for j in 0..m {
            for i in 0..n {
                data.push(mem::take(&mut self[(i, j)]));
            }
        }
        Self { data, shape: (m, n) }
    }

    pub fn to_string_unbroken(&self) -> String
    where
        T: fmt::Display,
    {
        (0..self.num_rows()).map(|i| {
            (0..self.num_cols()).map(|j| {
                self[(i, j)].to_string()
            }).join("")
        }).join("\n")
    }

    pub fn to_string_separated(&self, sep: &str) -> String
    where
        T: fmt::Display,
    {
        (0..self.num_rows()).map(|i| {
            (0..self.num_cols()).map(|j| {
                self[(i, j)].to_string()
            }).join(sep)
        }).join("\n")
    }

    pub fn to_string_justified(&self) -> String
    where
        T: fmt::Display,
    {
        let mut max_col_len = vec![0; self.num_cols()];
        for i in 0..self.num_rows() {
            for j in 0..self.num_cols() {
                max_col_len[j].relax_max(self[(i, j)].to_string().len());
            }
        }
        (0..self.num_rows()).map(|i| {
            (0..self.num_cols()).map(|j| {
                format!("{:>width$}", self[(i, j)], width = max_col_len[j])
            }).join(" ")
        }).join("\n")
    }
}

// Improvement potential: multiline output with `{:#?}`.
impl<T: fmt::Debug> fmt::Debug for Array2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Array2D[{}]", self.rows().map(|row|
            format!("[{}]", row.iter().map(|v| format!("{v:?}")).join(", "))
        ).join(", "))
    }
}

impl<T> From<Vec<Vec<T>>> for Array2D<T> {
    fn from(data: Vec<Vec<T>>) -> Self {
        let n_rows = data.len();
        let n_cols = data.first().map_or(0, |row| row.len());
        assert!(data.iter().all(|row| row.len() == n_cols));
        let data = data.into_iter().flatten().collect();
        Self { data, shape: (n_rows, n_cols) }
    }
}

impl<T> ops::Index<(usize, usize)> for Array2D<T> {
    type Output = T;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        let (n_rows, n_cols) = self.shape;
        assert!(row < n_rows && col < n_cols);
        &self.data[row * n_cols + col]
    }
}

impl<T> ops::IndexMut<(usize, usize)> for Array2D<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        let (n_rows, n_cols) = self.shape;
        assert!(row < n_rows && col < n_cols);
        &mut self.data[row * n_cols + col]
    }
}

pub trait Array2DReading<T> {
    fn array2d(&mut self, n_rows: usize, n_cols: usize) -> Array2D<T>;
}

impl<T, R> Array2DReading<T> for Reader<R>
where
    T: str::FromStr,
    <T as str::FromStr>::Err: fmt::Debug,
    R: BufRead,
{
    fn array2d(&mut self, n_rows: usize, n_cols: usize) -> Array2D<T> {
        let len = n_rows * n_cols;
        let mut data = Vec::with_capacity(len);
        for _ in 0..len {
            data.push(self.atom());
        }
        Array2D { data, shape: (n_rows, n_cols) }
    }
}

pub trait CharArray2DReading {
    fn char_array2d(&mut self, n_rows: usize, n_cols: usize) -> Array2D<char>;
}

impl<R> CharArray2DReading for Reader<R>
where
    R: BufRead,
{
    fn char_array2d(&mut self, n_rows: usize, n_cols: usize) -> Array2D<char> {
        let len = n_rows * n_cols;
        let mut data = Vec::with_capacity(len);
        for _ in 0..n_rows {
            let mut num_chars = 0;
            for ch in self.word().chars() {
                num_chars += 1;
                data.push(ch);
            }
            assert_eq!(num_chars, n_cols);
        }
        Array2D { data, shape: (n_rows, n_cols) }
    }
}


#[cfg(test)]
mod tests {
    use crate::internal_testing::{reader_from_string, catch_unwind_silent};
    use super::*;

    #[test]
    fn to_string() {
        let mut reader = reader_from_string("1 2 3\n4 555 6\n");
        let a: Array2D<i32> = reader.array2d(2, 3);
        assert_eq!(a.to_string_unbroken(), "123\n45556");
        assert_eq!(a.to_string_justified(), "1   2 3\n4 555 6");
        assert_eq!(a.to_string_separated(", "), "1, 2, 3\n4, 555, 6");
    }

    #[test]
    fn read_char_array() {
        let mut reader = reader_from_string("abc\ndef\n");
        let a: Array2D<char> = reader.char_array2d(2, 3);
        assert_eq!(a[(0, 0)], 'a');
        assert_eq!(a[(1, 2)], 'f');
    }

    #[test]
    fn mapping() {
        let a: Array2D<_> = vec![vec![1, 2, 3], vec![4, 5, 6]].into();
        let b = a.clone().map(|x| x * 2);
        assert_eq!(b, vec![vec![2, 4, 6], vec![8, 10, 12]].into());
        let c = a.map_enumerated(|(r, c), x| r * 100 + c * 10 + x);
        assert_eq!(c, vec![vec![1, 12, 23], vec![104, 115, 126]].into());
    }

    #[test]
    fn iteration() {
        let a: Array2D<_> = vec![vec![1, 2, 3], vec![4, 5, 6]].into();
        let mut rows = a.rows();
        assert_eq!(rows.next(), Some(&[1, 2, 3][..]));
        assert_eq!(rows.next(), Some(&[4, 5, 6][..]));
        assert_eq!(rows.next(), None);
    }

    #[test]
    fn index_check() {
        let mut a = Array2D::<i32>::with_shape(2, 2);
        assert!(catch_unwind_silent(move || a[(0, 2)] = 1).is_err());
    }

    #[test]
    fn debug_print() {
        let a: Array2D<_> = vec![vec![1, 2], vec![3, 4]].into();
        assert_eq!(format!("{:?}", a), "Array2D[[1, 2], [3, 4]]");
    }
}
