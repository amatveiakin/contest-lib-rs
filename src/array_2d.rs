// A 2D array. Just an array, with no matrix operations.
//
// TODO: Split `Array2D`, `DynArray2D` and `FixedArray2D` into files, similarly to
//   graph.rs /directed_graph.rs / undirected_graph.rs.

use std::io::BufRead;
use std::{fmt, ops, str};

use crate::io::Reader;
use crate::iterutils::Iterutils;
use crate::relax::RelaxMinMax;


pub trait Array2D<T> : ops::Index<[usize; 2], Output = T> + ops::IndexMut<[usize; 2]> {
    type MapResult<U>: Array2D<U>;
    type TransposeResult: Array2D<T>;

    fn num_rows(&self) -> usize;
    fn num_cols(&self) -> usize;
    fn shape(&self) -> (usize, usize);

    fn map<U>(self, f: impl Fn(T) -> U) -> Self::MapResult<U>;
    fn map_enumerated<U>(self, f: impl Fn([usize; 2], T) -> U) -> Self::MapResult<U>;

    // Improvement potential: Move `iter_enumerated` here.

    // Improvement potential: Benchmark if intermediate mapping to `Option` slows dows `transpose` a
    // lot. If it does, replace with an unsafe solution or require `T: Default + Copy`.
    fn transpose(self) -> Self::TransposeResult;

    fn to_string_unbroken(&self) -> String
    where
        T: fmt::Display,
    {
        (0..self.num_rows()).map(|i| {
            (0..self.num_cols()).map(|j| {
                self[[i, j]].to_string()
            }).join("")
        }).join("\n")
    }

    fn to_string_separated(&self, sep: &str) -> String
    where
        T: fmt::Display,
    {
        (0..self.num_rows()).map(|i| {
            (0..self.num_cols()).map(|j| {
                self[[i, j]].to_string()
            }).join(sep)
        }).join("\n")
    }

    fn to_string_justified(&self) -> String
    where
        T: fmt::Display,
    {
        let mut max_col_len = vec![0; self.num_cols()];
        for i in 0..self.num_rows() {
            for j in 0..self.num_cols() {
                max_col_len[j].relax_max(self[[i, j]].to_string().len());
            }
        }
        (0..self.num_rows()).map(|i| {
            (0..self.num_cols()).map(|j| {
                format!("{:>width$}", self[[i, j]], width = max_col_len[j])
            }).join(" ")
        }).join("\n")
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DynArray2D<T> {
    // Invariant: data.len() == shape.0 * shape.1
    data: Vec<T>,
    shape: (usize, usize),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FixedArray2D<T, const ROWS: usize, const COLS: usize> {
    // Note: Cannot write `[T; ROWS * COLS]` generic parameters cannot be used in const expressions
    // yet (see https://blog.rust-lang.org/inside-rust/2021/09/06/Splitting-const-generics.html,
    // `generic_const_exprs`)
    data: [[T; COLS]; ROWS],
}

impl<T> DynArray2D<T> {
    pub fn new(n_rows: usize, n_cols: usize) -> Self
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

    pub fn iter_enumerated(&self) -> impl Iterator<Item = ([usize; 2], &T)> {
        let (_, n_cols) = self.shape;
        self.data.iter().enumerate().map(move |(i, x)| {
            ([i / n_cols, i % n_cols], x)
        })
    }
}

impl<T, const ROWS: usize, const COLS: usize> FixedArray2D<T, ROWS, COLS> {
    pub fn new() -> Self
    where
        T: Default + Copy,
    {
        Self { data: [[T::default(); COLS]; ROWS] }
    }

    pub fn iter_enumerated(&self) -> impl Iterator<Item = ([usize; 2], &T)> {
        (0..self.num_rows()).flat_map(move |row| {
            (0..self.num_cols()).map(move |col| {
                ([row, col], &self[[row, col]])
            })
        })
    }
}

impl<T> Array2D<T> for DynArray2D<T> {
    type MapResult<U> = DynArray2D<U>;
    type TransposeResult = DynArray2D<T>;

    fn num_rows(&self) -> usize { self.shape.0 }
    fn num_cols(&self) -> usize { self.shape.1 }
    fn shape(&self) -> (usize, usize) { self.shape }

    fn map<U>(self, f: impl Fn(T) -> U) -> DynArray2D<U> {
        let data = self.data.into_iter().map(|x| f(x)).collect();
        DynArray2D { data, shape: self.shape }
    }
    fn map_enumerated<U>(self, f: impl Fn([usize; 2], T) -> U) -> DynArray2D<U> {
        let (_, n_cols) = self.shape;
        let data = self.data.into_iter().enumerate().map(|(i, x)| f([i / n_cols, i % n_cols], x)).collect();
        DynArray2D { data, shape: self.shape }
    }

    fn transpose(self) -> Self::TransposeResult {
        let (n, m) = self.shape;
        let mut old_data = self.data.into_iter().map(|x| Some(x)).collect::<Vec<_>>();
        let mut data = Vec::with_capacity(n * m);
        for j in 0..m {
            for i in 0..n {
                data.push(old_data[i * m + j].take().unwrap());
            }
        }
        Self { data, shape: (m, n) }
    }
}

impl<T, const ROWS: usize, const COLS: usize> Array2D<T> for FixedArray2D<T, ROWS, COLS> {
    type MapResult<U> = FixedArray2D<U, ROWS, COLS>;
    type TransposeResult = FixedArray2D<T, COLS, ROWS>;

    fn num_rows(&self) -> usize { ROWS }
    fn num_cols(&self) -> usize { COLS }
    fn shape(&self) -> (usize, usize) { (ROWS, COLS) }

    fn map<U>(self, f: impl Fn(T) -> U) -> FixedArray2D<U, ROWS, COLS> {
        let data = self.data.map(|row_data| row_data.map(|x| f(x)));
        FixedArray2D { data }
    }
    fn map_enumerated<U>(self, f: impl Fn([usize; 2], T) -> U) -> FixedArray2D<U, ROWS, COLS> {
        let mut row = 0;
        let mut col = 0;
        let data = self.data.map(|row_data| {
            let ret = row_data.map(|x| {
                let ret = f([row, col], x);
                col += 1;
                ret
            });
            row += 1;
            col = 0;
            ret
        });
        FixedArray2D { data }
    }

    fn transpose(self) -> Self::TransposeResult {
        let mut old_data = self.data.map(|row_data| row_data.map(|x| Some(x)));
        let mut data = [[(); ROWS]; COLS].map(|row_data| row_data.map(|()| None));
        for i in 0..ROWS {
            for j in 0..COLS {
                data[j][i] = old_data[i][j].take();
            }
        }
        let data = data.map(|row_data| row_data.map(|x| x.unwrap()));
        Self::TransposeResult { data }
    }
}

// Improvement potential: multiline output with `{:#?}`.
fn debug_print_array2d<T: fmt::Debug>(title: &str, a: &impl Array2D<T>, f: &mut fmt::Formatter)
    -> fmt::Result
{
    write!(f, "{title}[{}]", (0..a.num_rows()).map(|row|
        format!("[{}]", (0..a.num_cols()).map(|col| format!("{:?}", a[[row, col]])).join(", "))
    ).join(", "))
}

impl<T: fmt::Debug> fmt::Debug for DynArray2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_print_array2d("DynArray2D", self, f)
    }
}

impl<T: fmt::Debug, const ROWS: usize, const COLS: usize> fmt::Debug for FixedArray2D<T, ROWS, COLS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_print_array2d("FixedArray2D", self, f)
    }
}

impl<T> From<Vec<Vec<T>>> for DynArray2D<T> {
    fn from(data: Vec<Vec<T>>) -> Self {
        let n_rows = data.len();
        let n_cols = data.first().map_or(0, |row| row.len());
        assert!(data.iter().all(|row| row.len() == n_cols));
        let data = data.into_iter().flatten().collect();
        Self { data, shape: (n_rows, n_cols) }
    }
}

impl<T, const ROWS: usize, const COLS: usize> From<[[T; COLS]; ROWS]> for FixedArray2D<T, ROWS, COLS> {
    fn from(data: [[T; COLS]; ROWS]) -> Self {
        Self { data }
    }
}

impl<T> ops::Index<[usize; 2]> for DynArray2D<T> {
    type Output = T;
    fn index(&self, [row, col]: [usize; 2]) -> &Self::Output {
        let (n_rows, n_cols) = self.shape;
        assert!(row < n_rows && col < n_cols);
        &self.data[row * n_cols + col]
    }
}
impl<T> ops::IndexMut<[usize; 2]> for DynArray2D<T> {
    fn index_mut(&mut self, [row, col]: [usize; 2]) -> &mut Self::Output {
        let (n_rows, n_cols) = self.shape;
        assert!(row < n_rows && col < n_cols);
        &mut self.data[row * n_cols + col]
    }
}

impl<T, const ROWS: usize, const COLS: usize> ops::Index<[usize; 2]> for FixedArray2D<T, ROWS, COLS> {
    type Output = T;
    fn index(&self, [row, col]: [usize; 2]) -> &Self::Output {
        assert!(row < ROWS && col < COLS);
        &self.data[row][col]
    }
}
impl<T, const ROWS: usize, const COLS: usize> ops::IndexMut<[usize; 2]> for FixedArray2D<T, ROWS, COLS> {
    fn index_mut(&mut self, [row, col]: [usize; 2]) -> &mut Self::Output {
        assert!(row < ROWS && col < COLS);
        &mut self.data[row][col]
    }
}

pub trait Array2DReading<T> {
    fn array2d(&mut self, n_rows: usize, n_cols: usize) -> DynArray2D<T>;
}

impl<T, R> Array2DReading<T> for Reader<R>
where
    T: str::FromStr,
    <T as str::FromStr>::Err: fmt::Debug,
    R: BufRead,
{
    fn array2d(&mut self, n_rows: usize, n_cols: usize) -> DynArray2D<T> {
        let len = n_rows * n_cols;
        let mut data = Vec::with_capacity(len);
        for _ in 0..len {
            data.push(self.atom());
        }
        DynArray2D { data, shape: (n_rows, n_cols) }
    }
}

pub trait CharArray2DReading {
    fn char_array2d(&mut self, n_rows: usize, n_cols: usize) -> DynArray2D<char>;
}

impl<R> CharArray2DReading for Reader<R>
where
    R: BufRead,
{
    fn char_array2d(&mut self, n_rows: usize, n_cols: usize) -> DynArray2D<char> {
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
        DynArray2D { data, shape: (n_rows, n_cols) }
    }
}


#[cfg(test)]
mod tests {
    use crate::assert_panics;
    use crate::testing::io_utils::reader_from_string;
    use super::*;

    #[test]
    fn to_string() {
        let mut reader = reader_from_string("1 2 3\n4 555 6\n");
        let a: DynArray2D<i32> = reader.array2d(2, 3);
        assert_eq!(a.to_string_unbroken(), "123\n45556");
        assert_eq!(a.to_string_justified(), "1   2 3\n4 555 6");
        assert_eq!(a.to_string_separated(", "), "1, 2, 3\n4, 555, 6");
    }

    #[test]
    fn read_char_array() {
        let mut reader = reader_from_string("abc\ndef\n");
        let a: DynArray2D<char> = reader.char_array2d(2, 3);
        assert_eq!(a[[0, 0]], 'a');
        assert_eq!(a[[1, 2]], 'f');
    }

    #[test]
    fn mapping_dyn() {
        let a: DynArray2D<_> = vec![vec![1, 2, 3], vec![4, 5, 6]].into();
        let b = a.clone().map(|x| x * 2);
        assert_eq!(b, vec![vec![2, 4, 6], vec![8, 10, 12]].into());
        let c = a.map_enumerated(|[r, c], x| r * 100 + c * 10 + x);
        assert_eq!(c, vec![vec![1, 12, 23], vec![104, 115, 126]].into());
    }

    #[test]
    fn mapping_fixed() {
        let a: FixedArray2D<_, 2, 3> = [[1, 2, 3], [4, 5, 6]].into();
        let b = a.clone().map(|x| x * 2);
        assert_eq!(b, [[2, 4, 6], [8, 10, 12]].into());
        let c = a.map_enumerated(|[r, c], x| r * 100 + c * 10 + x);
        assert_eq!(c, [[1, 12, 23], [104, 115, 126]].into());
    }

    #[test]
    fn iteration() {
        let a: DynArray2D<_> = vec![vec![1, 2, 3], vec![4, 5, 6]].into();
        let mut rows = a.iter_enumerated();
        assert_eq!(rows.next(), Some(([0, 0], &1)));
        assert_eq!(rows.next(), Some(([0, 1], &2)));
        assert_eq!(rows.next(), Some(([0, 2], &3)));
        assert_eq!(rows.next(), Some(([1, 0], &4)));
        assert_eq!(rows.next(), Some(([1, 1], &5)));
        assert_eq!(rows.next(), Some(([1, 2], &6)));
        assert_eq!(rows.next(), None);
    }

    #[test]
    fn transposition_dyn() {
        let a: DynArray2D<_> = vec![vec![1, 2, 3], vec![4, 5, 6]].into();
        assert_eq!(a.transpose(), vec![vec![1, 4], vec![2, 5], vec![3, 6]].into());
    }

    #[test]
    fn transposition_fixed() {
        let a: FixedArray2D<_, 2, 3> = [[1, 2, 3], [4, 5, 6]].into();
        assert_eq!(a.transpose(), [[1, 4], [2, 5], [3, 6]].into());
    }

    #[test]
    fn index_check() {
        let mut a = DynArray2D::<i32>::new(2, 2);
        assert_panics!(move || a[[0, 2]] = 1);
    }

    #[test]
    fn debug_print_dyn() {
        let a: DynArray2D<_> = vec![vec![1, 2], vec![3, 4]].into();
        assert_eq!(format!("{:?}", a), "DynArray2D[[1, 2], [3, 4]]");
    }

    #[test]
    fn debug_print_fixed() {
        let a: FixedArray2D<_, 2, 2> = [[1, 2], [3, 4]].into();
        assert_eq!(format!("{:?}", a), "FixedArray2D[[1, 2], [3, 4]]");
    }
}
