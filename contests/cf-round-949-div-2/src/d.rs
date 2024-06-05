// UNFINISHED

use std::collections::{BTreeSet, HashMap};

use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::primes::primes;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();

    let mut free = HashMap::new();
    free.insert(1, BTreeSet::new());
    let mut max = 1;
    let mut last = 1;
    let mut a = vec![last, last];
    while a.len() < n {
        if let Some(next) = free.get_mut(&last).unwrap().pop_last() {
            a.push(next);
            last = next;
        } else {
            max += 1;
            a.push(max);
            let mut free_pairs = BTreeSet::from_iter(1..=max);
            free_pairs.remove(&last);
            free.insert(max, free_pairs);
            last = max;
        }
        // println!("{}: #{} (vs {}): {:?}", max, free.len(), max * (max + 1) / 2, a);
    }
    emitln!(write, a.into_iter().map(|i| primes().nth(i - 1).unwrap()).collect_vec());
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
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 3
        // 2
        // 3
        // 4
        // "), "\
        // 114514 114514
        // 1 2 2
        // 3 3 4 4");
        assert_trimmed_eq!(&run_solver(solve, "\
        3
        2
        3
        4
        "), "\
        2 2
        2 2 3
        2 2 3 3");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}


// Helpers:
/*
use std::collections::HashSet;

use contest_lib_rs::relax::Relax;
use contest_lib_rs::sort_array::sort_array;

fn exhaustive_search(max: u32, arr: &mut Vec<u32>, pairs: &mut HashSet<[u32; 2]>) -> usize {
    let mut best = arr.len() - 1;
    for i in 1..=max {
        let p = sort_array([i, *arr.last().unwrap()]);
        if pairs.insert(p) {
            arr.push(i);
            best.relax_max(exhaustive_search(max, arr, pairs));
            arr.pop();
            pairs.remove(&p);
        }
    }
    best
}

fn check(a: &[u32]) {
    let mut pairs = HashSet::new();
    for i in 0..(a.len() - 1) {
        let p = sort_array([a[i], a[i + 1]]);
        assert!(pairs.insert(p), "{p:?}");
    }
    println!("OK: #{}: {:?}", pairs.len(), a);
}

fn main() {
    let mut pairs = HashSet::new();
    pairs.insert([1, 1]);
    let mut max = 1;
    let mut last = 1;
    let mut a = vec![last, last];
    'outer: while max <= 6 {
        let total = max * (max + 1) / 2;
        let best = {
            let mut search_pairs = HashSet::new();
            let mut search_arr = vec![1];
            exhaustive_search(max, &mut search_arr, &mut search_pairs)
        };
        for i in (1..=max).rev() {
            let p = sort_array([i, last]);
            if !pairs.contains(&p) {
                pairs.insert(p);
                last = i;
                a.push(i);
                continue 'outer;
            }
        }
        let num_pairs = pairs.len();
        println!("{max}: #{num_pairs} (of {best} / {total}): {a:?}");
        if max % 2 == 1 {
            assert!(pairs.len() == total as usize)
        }
        max += 1;
    }
    check(&[1, 1, 2, 2, 3, 3, 1,   4, 4, 5, 5, 6, 6, 4,   2, 5, 1, 6, 3, 4]);
}
*/