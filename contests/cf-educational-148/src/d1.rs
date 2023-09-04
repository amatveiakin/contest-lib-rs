use std::cmp;

use contest_lib_rs::{io, emit, counting_set::CountingSet};

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let q = read.usize();
    let mut a = read.vec_i64(n);
    a.sort();
    for _ in 0..q {
        let k = read.usize();
        let mut final_operations = cmp::min(k, n);
        let mut intermediate_operations = k - final_operations;
        if intermediate_operations % 2 != 0 {
            final_operations -= 1;
            intermediate_operations += 1;
        }
        let mut operations_left = intermediate_operations / 2;
        let mut a = a.clone();
        for i in 0..final_operations {
            a[i] += (k - i) as i64;
        }
        let mut h: CountingSet<_> = a.into_iter().collect();
        while operations_left > 0 {
            let (value, count) = h.pop_last_group().unwrap();
            let second = h.last();
            let operations_applied = cmp::min(
                operations_left,
                usize::try_from(value - second.unwrap_or(&-1000_000_000)).unwrap() * count,
            );
            if operations_applied % count == 0 {
                let value = value - (operations_applied / count) as i64;
                h.push_multiple(value, count);
            } else {
                let value = value - (operations_applied / count) as i64;
                h.push_multiple(value, count - operations_applied % count);
                h.push_multiple(value - 1, operations_applied % count);
            }
            operations_left -= operations_applied;
        }
        emit!(write, h.first().unwrap());
    }
}

fn main() {
    let mut read = io::Reader::new(std::io::stdin().lock());
    let mut write = std::io::stdout().lock();
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
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 2 1
        // 1 1
        // 3
        // "), "1");
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 2 1
        // 1 100
        // 3
        // "), "4");
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 4 1
        // 5 2 8 4
        // 5
        // "), "7");


        assert_trimmed_eq!(&run_solver(solve, "\
        4 10
        5 2 8 4
        1 2 3 4 5 6 7 8 9 10
        "), "3 4 5 6 7 8 8 10 8 12");
        assert_trimmed_eq!(&run_solver(solve, "\
        5 10
        5 2 8 4 4
        1 2 3 4 5 6 7 8 9 10
        "), "3 4 5 6 7 8 9 8 11 8");
        assert_trimmed_eq!(&run_solver(solve, "\
        2 5
        2 3
        10 6 8 1 3
        "), "10 7 8 3 3");
    }
}
