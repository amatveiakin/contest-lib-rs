use std::collections::BTreeSet;

use contest_lib_rs::btreeset_util::OrderedSetNeighborValues;
use contest_lib_rs::counting_set::CountingSet;
use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.i64();
    let a = read.vec_i64(n as usize);
    let mut m = read.i64();
    let mut a = a.into_iter().enumerate().map(|(p, v)| (v, p)).collect::<Vec<_>>();
    a.sort();
    a.reverse();

    let mut ap: usize = 0;
    let mut cols = BTreeSet::<i32>::new();
    let mut row_segments = CountingSet::<i32>::new();
    let mut total_segments = vec![0i64; n as usize + 1];
    row_segments.push(n as i32);

    for row in (1..=n).rev() {
        while (ap as i64) < n && a[ap].0 >= row {
            let p = a[ap].1 as i32;
            cols.insert(p);
            let prev = *cols.prev_value(&p).unwrap_or(&-1);
            let next = *cols.next_value(&p).unwrap_or(&(n as i32));
            let len = next - prev - 1;
            if len > 1 {
                row_segments.remove(len);
            }
            let (len1, len2) = (p - prev - 1, next - p - 1);
            for l in [len1, len2].iter() {
                if *l > 1 {
                    row_segments.push(*l);
                }
            }
            ap += 1;
        }
        for (len, count) in row_segments.iter_groups() {
            total_segments[*len as usize] += count as i64;
        }
    }

    let mut answer: i64 = 0;
    for (len, &count) in total_segments.iter().enumerate().rev() {
        if count == 0 || len == 0 {
            continue;
        }
        let len = len as i64;
        let count = count as i64;
        if m >= count * len {
            m -= count * len;
            answer += count * (len - 1);
        } else {
            let num_whole = m / len;
            let partial_len = m % len;
            answer += num_whole * (len - 1);
            if partial_len > 0 {
                answer += partial_len - 1;
            }
            m = 0;
        }
        if m == 0 {
            break;
        }
    }

    emitln!(write, answer);
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let t = read.usize();
    for _ in 0..t {
        solve_case(read, write);
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
    // use pretty_assertions::assert_eq;
    use contest_lib_rs::{solution_testing::run_solver, assert_trimmed_eq};

    #[test]
    fn test() {
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 1
        // 10
        // 0 2 2 1 5 10 3 4 1 1
        // 20"), "16");

        assert_trimmed_eq!(&run_solver(solve, "\
        6
        3
        0 0 0
        9
        4
        2 0 3 1
        5
        4
        2 0 3 1
        6
        4
        2 0 3 1
        10
        10
        0 2 2 1 5 10 3 4 1 1
        20
        1
        1
        0
        "), "\
        6
        3
        4
        4
        16
        0");

        assert_trimmed_eq!(&run_solver(solve, "\
        1
        2
        0 0
        4
        "),
        "2");
    }
}
