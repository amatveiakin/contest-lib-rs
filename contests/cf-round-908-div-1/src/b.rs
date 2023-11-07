// UNFINISHED

use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;

fn longest_increasing_subsequence<T: Ord + Default + Copy>(a: &[T]) -> Vec<T> {
    let n = a.len();
    let mut p = vec![0; n];
    let mut m = vec![0; n + 1];
    m[0] = usize::MAX;

    let mut l = 0;
    for i in 0..n {
        let mut lo = 1;
        let mut hi = l + 1;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if a[m[mid]] >= a[i] {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        let new_l = lo;

        p[i] = m[new_l - 1];
        m[new_l] = i;

        if new_l > l {
            l = new_l;
        }
    }

    let mut s = vec![T::default(); l];
    let mut k = m[l];
    for j in (0..l).rev() {
        s[j] = a[k];
        k = p[k];
    }
    s
}

fn solve_impl(a: &[u32], b: &[u32]) -> Vec<u32> {
    let al = longest_increasing_subsequence(&a);
    assert!(al.len() > 0);
    if al.len() == 1 {
        let mut c = a.iter().chain(b.iter()).copied().collect_vec();
        c.sort_by_key(|x| std::cmp::Reverse(*x));
        c
    } else {
        let mut b = b.to_vec();
        b.sort_by_key(|x| std::cmp::Reverse(*x));
        // let q = al[0];
        let q = al.last().unwrap();
        b.iter().filter(|&x| x > &q)
            .chain(a.iter())
            .chain(b.iter().filter(|&x| x <= &q))
            .copied()
            .collect_vec()
    }
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m] = read.usizes();
    let a = read.vec_u32(n);
    let b = read.vec_u32(m);
    let c = solve_impl(&a, &b);
    emitln!(write, c);
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let t = read.usize();
    for _ in 0..t {
        solve_case(read, write);
    }
}

fn main() {
    let mut read = Reader::new(std::io::stdin().lock());
    let mut write = std::io::BufWriter::new(std::io::stdout().lock());
    solve(&mut read, &mut write);
}


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use contest_lib_rs::rand::{self, Rng};
    use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test_lis() {
        assert_eq!(longest_increasing_subsequence(&[1, 2, 3, 4, 5, 6, 7, 8, 9]), [1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(longest_increasing_subsequence(&[5, 4, 3, 2, 1]).len(), 1);
        assert_eq!(longest_increasing_subsequence(&[1, 1, 2, 2, 1, 3, 2, 3, 1]), [1, 2, 3]);
        assert_eq!(longest_increasing_subsequence(&[6, 5, 3, 2, 1, 9, 8, 3, 4]).len(), 3);
        assert_eq!(longest_increasing_subsequence(&[1, 1, 1, 1]), [1]);
        assert_eq!(longest_increasing_subsequence(&[10]), [10]);
        assert_eq!(longest_increasing_subsequence(&[8, 7, 6, 5, 4, 1, 9, 2, 3, 8, 1, 4, 7, 2, 9]).len(), 6);
        assert_eq!(longest_increasing_subsequence(&[1, 9, 2, 3, 8, 1, 4, 7, 2, 9, 8, 7, 6, 5, 4]).len(), 6);
        assert_eq!(longest_increasing_subsequence(&[1, 9, 2, 3, 8, 8, 1, 4, 4, 7, 7, 2, 9, 6, 5]).len(), 6);
        assert_eq!(longest_increasing_subsequence(&[1, 3, 5, 4, 2]).len(), 3);
        assert_eq!(longest_increasing_subsequence(&[1, 3, 5, 2, 4]).len(), 3);
    }

    // #[test]
    // fn test_rnd() {
    //     loop {
    //         let rng = &mut rand::thread_rng();
    //         let n: u32 = rng.int_range_inclusive(1, 20);
    //         let m: u32 = rng.int_range_inclusive(1, 20);
    //         let mut a = vec![];
    //         let mut b = vec![];
    //         for _ in 0..n {
    //             a.push(rng.int_range_inclusive(1, 10));
    //         }
    //         for _ in 0..m {
    //             b.push(rng.int_range_inclusive(1, 10));
    //         }
    //         println!("a = {a:?}, b = {b:?}");
    //         let c = solve_impl(&a, &b);
    //         assert_eq!(
    //             longest_increasing_subsequence(&a).len(),
    //             longest_increasing_subsequence(&c).len(),
    //             "a = {a:?}, b = {b:?}, c = {c:?}",
    //         );
    //     }
    // }

    // #[test]
    // fn test_my() {
    //     // let a = vec![4, 2, 8, 6];
    //     // let b = vec![1, 3, 5, 7, 9];
    //
    //     let a = vec![6, 2, 8, 4];
    //     // let b = vec![1, 3, 5, 7, 9];
    //     let b = vec![5];
    //
    //     let c = solve_impl(&a, &b);
    //     println!("c = {c:?}");
    //     assert_eq!(longest_increasing_subsequence(&a).len(), longest_increasing_subsequence(&c).len());
    // }

    // #[test]
    // fn test() {
    //     assert_trimmed_eq!(&run_solver(solve, "\
    //     7
    //     2 1
    //     6 4
    //     5
    //     5 5
    //     1 7 2 4 5
    //     5 4 1 2 7
    //     1 9
    //     7
    //     1 2 3 4 5 6 7 8 9
    //     3 2
    //     1 3 5
    //     2 4
    //     10 5
    //     1 9 2 3 8 1 4 7 2 9
    //     7 8 5 4 6
    //     2 1
    //     2 2
    //     1
    //     6 1
    //     1 1 1 1 1 1
    //     777
    //     "), "\
    //     6 5 4
    //     1 1 7 7 2 2 4 4 5 5
    //     9 8 7 7 6 5 4 3 2 1
    //     1 3 5 2 4
    //     1 9 2 3 8 8 1 4 4 7 7 2 9 6 5
    //     2 2 1
    //     777 1 1 1 1 1 1");
    // }
}
