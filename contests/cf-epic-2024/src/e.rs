use std::collections::{BTreeMap, HashSet};

use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::relax::Relax;

#[derive(Debug)]
struct Group {
    len: u64,
    val: u32,
    protected_since: Option<u64>,
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let mut gr = (0..n).map(|_| {
        let len = read.u64();
        let val = read.u32();
        Group { len, val, protected_since: None }
    }).collect_vec();

    let mut prev_live = vec![None; n];
    for i in 1..n {
        prev_live[i] = Some(i - 1);
    }
    let mut next_live = vec![None; n];
    for i in 0..(n - 1) {
        next_live[i] = Some(i + 1);
    }

    let mut unprotected_lens = BTreeMap::new();
    for i in 0..n {
        unprotected_lens.entry(gr[i].len).or_insert_with(HashSet::new).insert(i);
    }

    let mut prev_len = 0;
    let mut t = 0;

    let mut death_t = vec![None; n];

    while let Some((len, lenp)) = unprotected_lens.pop_first() {
        t += len - prev_len;
        prev_len = len;

        let mut upd = HashSet::new();
        for p in lenp {
            assert!(death_t[p].is_none());
            death_t[p] = Some(t);
            upd.remove(&p);
            if let Some(next) = next_live[p] {
                prev_live[next] = prev_live[p];
                upd.insert(next);
            }
            if let Some(prev) = prev_live[p] {
                next_live[prev] = next_live[p];
            }
        }

        for i in upd {
            if let Some(old_bucket) = unprotected_lens.get_mut(&gr[i].len) {
                old_bucket.remove(&i);
            }

            let protected = if let Some(prev) = prev_live[i] {
                gr[i].val == gr[prev].val
            } else {
                false
            };

            match (gr[i].protected_since, protected) {
                (Some(_), true) => {},
                (Some(protected_since), false) => {
                    gr[i].len += t - protected_since;
                    gr[i].protected_since = None;
                },
                (None, true) => {
                    gr[i].protected_since = Some(t);
                }
                (None, false) => {},
            }
            if !protected {
                unprotected_lens.entry(gr[i].len).or_insert_with(HashSet::new).insert(i);
            }
        }
    }

    let mut answer = vec![0; n];
    let mut max_t = 0;
    for i in 0..n {
        max_t.relax_max(death_t[i].unwrap());
        answer[i] = max_t;
    }
    emitln!(write, answer);
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
        // assert_trimmed_eq!(&run_solver(solve_case, ""), "");
        // assert_trimmed_eq!(&run_solver(solve_case, "1  10 42"), "10");
        // assert_trimmed_eq!(&run_solver(solve_case, "2  1 10  2 20"), "1 2");
        // assert_trimmed_eq!(&run_solver(solve_case, "\
        // 10
        // 10 7
        // 4 9
        // 2 2
        // 7 9
        // 2 8
        // 8 5
        // 11 7
        // 15 5
        // 12 7
        // 4 0
        // "),
        // "");
        assert_trimmed_eq!(&run_solver(solve, "\
        4
        4
        2 0
        1 1
        3 0
        5 1
        6
        4 6
        1 3
        4 6
        4 0
        7 6
        6 3
        7
        9 0
        7 1
        5 0
        7 1
        9 0
        1 1
        2 0
        10
        10 7
        4 9
        2 2
        7 9
        2 8
        8 5
        11 7
        15 5
        12 7
        4 0"), "\
        2 2 4 5
        4 4 7 7 10 10
        9 9 9 9 9 9 10
        10 10 10 10 10 10 12 15 15 15 ");
    }
}
