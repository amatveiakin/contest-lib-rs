use std::collections::{HashMap, HashSet};
use std::ops;

use crate::num::RingInteger;


pub struct SparseMatrix<T: RingInteger> {
    rows: Vec<HashMap<usize, T>>,
    cols: Vec<HashSet<usize>>,
}

impl<T: RingInteger> SparseMatrix<T> {
    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        let rows = vec![HashMap::new(); num_rows];
        let cols = vec![HashSet::new(); num_cols];
        Self { rows, cols }
    }

    pub fn num_rows(&self) -> usize { self.rows.len() }
    pub fn num_cols(&self) -> usize { self.cols.len() }

    #[track_caller]
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        if value != T::zero() {
            self.rows[row].insert(col, value);
            self.cols[col].insert(row);
        } else {
            self.rows[row].remove(&col);
            self.cols[col].remove(&row);
        }
    }

    #[track_caller]
    pub fn get_mut(&mut self, row: usize, col: usize) -> ElementAccess<T> {
        let value = self[[row, col]];
        ElementAccess { matrix: self, row, col, value }
    }

    #[track_caller]
    pub fn row(&self, row: usize) -> impl ExactSizeIterator<Item = (usize, T)> + '_ {
        self.rows[row].iter().map(|(&col, &value)| (col, value))
    }

    #[track_caller]
    pub fn col_indices(&self, col: usize) -> impl ExactSizeIterator<Item = usize> + '_ {
        self.cols[col].iter().map(|&row| row)
    }
}

impl<T: RingInteger + 'static> ops::Index<[usize; 2]> for SparseMatrix<T> {
    type Output = T;
    #[track_caller]
    fn index(&self, [row, col]: [usize; 2]) -> &Self::Output {
        self.rows[row].get(&col).unwrap_or(T::zero_ref())
    }
}

pub struct ElementAccess<'a, T: RingInteger + 'static> {
    matrix: &'a mut SparseMatrix<T>,
    row: usize,
    col: usize,
    value: T,
}

impl<T: RingInteger + 'static> ops::Deref for ElementAccess<'_, T> {
    type Target = T;
    #[track_caller]
    fn deref(&self) -> &T {
        &self.matrix[[self.row, self.col]]
    }
}

impl<T: RingInteger + 'static> ops::DerefMut for ElementAccess<'_, T> {
    #[track_caller]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T: RingInteger + 'static> Drop for ElementAccess<'_, T> {
    fn drop(&mut self) {
        self.matrix.set(self.row, self.col, self.value);
    }
}


#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    fn into_sorted_vec<T: Ord>(iter: impl IntoIterator<Item = T>) -> Vec<T> {
        let mut vec = iter.into_iter().collect_vec();
        vec.sort();
        vec
    }

    #[test]
    fn basic() {
        let mut m = SparseMatrix::new(4, 4);
        m.set(0, 1, 10);
        m.set(0, 2, 20);
        *m.get_mut(0, 2) += 2;
        *m.get_mut(1, 2) += 30;
        *m.get_mut(3, 2) += 40;
        *m.get_mut(3, 2) -= 40;

        assert_eq!(into_sorted_vec(m.row(0)), [(1, 10), (2, 22)]);
        assert_eq!(into_sorted_vec(m.row(1)), [(2, 30)]);
        assert_eq!(into_sorted_vec(m.row(2)), []);
        assert_eq!(into_sorted_vec(m.row(3)), []);

        assert_eq!(into_sorted_vec(m.col_indices(0)), []);
        assert_eq!(into_sorted_vec(m.col_indices(1)), [0]);
        assert_eq!(into_sorted_vec(m.col_indices(2)), [0, 1]);
        assert_eq!(into_sorted_vec(m.col_indices(3)), []);
    }
}
