use std::collections::{HashMap, BTreeSet};

use contest_lib_rs::btreeset_util::OrderedSetNeighborValues;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::segment_tree::{new_max_tree, new_min_tree};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u32(n);
    let b = read.vec_u32(n);

    let mut amax = new_max_tree(&a);
    let mut bmin = new_min_tree(&b);

    let mut a_groups = HashMap::new();
    for (i, &x) in a.iter().enumerate() {
        a_groups.entry(x).or_insert_with(BTreeSet::new).insert(i as u32);
    }

    for (i, &x) in b.iter().enumerate() {
        if a[i] > x {
            emitln!(write, "NO");
            return;
        }
        if a[i] == x {
            continue;
        }
        let mut fixed = false;
        let i = i as u32;
        if let Some(gr) = a_groups.get(&x) {
            if let Some(&l) = gr.prev_value(&i) {
                if amax.get(l..=i) <= x && bmin.get(l..=i) >= x {
                    fixed = true;
                }
            }
            if let Some(&r) = gr.next_value(&i) {
                if amax.get(i..=r) <= x && bmin.get(i..=r) >= x {
                    fixed = true;
                }
            }
        }
        if !fixed {
            emitln!(write, "NO");
            return;
        }
    }
    emitln!(write, "YES");
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
        assert_trimmed_eq!(&run_solver(solve_case, "5  3 2 1 1 1  3 3 3 2 2"), "YES");
        assert_trimmed_eq!(&run_solver(solve_case, "3  1 1 2  2 1 2"), "NO");
        assert_trimmed_eq!(&run_solver(solve, "\
        5
        5
        1 2 3 2 4
        1 3 3 2 4
        5
        3 4 2 2 4
        3 4 3 4 4
        5
        3 2 1 1 1
        3 3 3 2 2
        2
        1 1
        1 2
        3
        1 1 2
        2 1 2
        "), "\
        YES
        NO
        YES
        NO
        NO");
    }
}
