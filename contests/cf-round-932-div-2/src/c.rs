use contest_lib_rs::binary_heaps::MaxHeap;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::relax::Relax;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let l = read.i64();
    let mut ab = vec![];
    for _ in 0..n {
        let [a, b] = read.i64s();
        ab.push((a, b));
    }
    ab.sort_by_key(|&(_, b)| std::cmp::Reverse(b));
    let (a, b): (Vec<_>, Vec<_>) = ab.into_iter().unzip();

    let mut best_k = 0;
    for i in 0..n {
        let mut taken = MaxHeap::new();
        let mut k = 0;
        let mut a_sum = 0;
        for j in i..n {
            let bv = b[i] - b[j];
            assert!(bv >= 0);
            let a_bugdet = l - bv;

            taken.push(a[j]);
            a_sum += a[j];
            k += 1;
            while a_sum > a_bugdet && !taken.is_empty() {
                let a_big = taken.pop().unwrap();
                a_sum -= a_big;
                k -= 1;
            }
            best_k.relax_max(k);
        }
    }
    emitln!(write, best_k);
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
    use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve_case, "\
        5 8
        4 3
        1 5
        2 4
        4 3
        2 3
        "), "3");
        assert_trimmed_eq!(&run_solver(solve_case, "\
        1 10
        5 20
        "), "1");
        assert_trimmed_eq!(&run_solver(solve_case, "\
        1 10
        20 5
        "), "0");
        assert_trimmed_eq!(&run_solver(solve, "\
        5
        5 8
        4 3
        1 5
        2 4
        4 3
        2 3
        1 6
        4 10
        3 12
        4 8
        2 1
        2 12
        5 26
        24 7
        8 28
        30 22
        3 8
        17 17
        5 14
        15 3
        1000000000 998244353
        179 239
        228 1337
        993 1007
        "), "\
        3
        1
        2
        1
        0");
    }
}
