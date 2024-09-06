use std::collections::{BTreeMap, BTreeSet, HashMap};

use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::btreeset_util::OrderedSetNeighborValues;
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, k] = read.usizes();
    let a = read.vec_usize(n).from1b();

    let mut p = vec![BTreeSet::new(); n];
    for i in 0..p.len() {
        p[i].insert(n + i + 1);
    }

    for (i, &x) in a.iter().enumerate() {
        p[x].insert(i);
    }

    let mut books = HashMap::new();
    let mut next_use = BTreeMap::new();
    let mut chf = 0;
    for (i, &x) in a.iter().enumerate() {
        let ni = p[x].next_value(&i).unwrap();
        if let Some(&j) = books.insert(x, ni) {
            assert_eq!(j, i);
            assert!(next_use.remove(&j).is_some());
        } else {
            chf += 1;
            if books.len() > k {
                let (&j, y) = next_use.pop_last().unwrap();
                assert!(j > i);
                assert_ne!(x, y);
                assert!(books.remove(&y).is_some());
            }
        }
        next_use.insert(ni, x);
    }
    emitln!(write, chf);
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
        assert_trimmed_eq!(&run_solver(solve, "4 80  1 2 2 1"), "2");
        assert_trimmed_eq!(&run_solver(solve, "4 1  1 2 2 1"), "3");
        assert_trimmed_eq!(&run_solver(solve, "4 2  1 2 3 1"), "3");
        assert_trimmed_eq!(&run_solver(solve, "7 2  1 2 3 3 2 1 3"), "4");
        assert_trimmed_eq!(&run_solver(solve, "10 1  7 4 4 7 7 7 7 3 7 7"), "5");
        assert_trimmed_eq!(&run_solver(solve, "10 3  1 2 3 4 1 2 3 4 1 2"), "6");
        assert_trimmed_eq!(&run_solver(solve, "10 2  1 2 3 4 1 2 3 4 1 2"), "8");
        assert_trimmed_eq!(&run_solver(solve, "10 2  1 2 3 4 5 6 7 8 9 10"), "10");
        assert_trimmed_eq!(&run_solver(solve, "10 4  1 2 3 4 5 6 7 8 9 10"), "10");
        assert_trimmed_eq!(&run_solver(solve, "13 3  1 2 3 4 5 6 1 2 3 1 2 3 1"), "7");
        assert_trimmed_eq!(&run_solver(solve, "10 2  1 2 3 4 5 6 1 2 1 2"), "7");
        assert_trimmed_eq!(&run_solver(solve, "10 2  1 2 1 2 1 2 3 4 5 6"), "6");
        assert_trimmed_eq!(&run_solver(solve, "6 2  1 2 1 2 3 2"), "3");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}