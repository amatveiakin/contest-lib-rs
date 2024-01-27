use std::collections::BTreeSet;

use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::bool_ext::BoolExtension;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::mod_ring::ModNumber;
use contest_lib_rs::segment_tree::{new_sum_tree, SegmentTree};

type M = ModNumber<26>;

fn set_assign<K: Eq + Ord>(set: &mut BTreeSet<K>, key: K, exists: bool) {
    if exists {
        set.insert(key);
    } else {
        set.remove(&key);
    }
}

// TODO: Reduce the number of generic parameters.
fn get_or<T: Clone, U: Clone, TC: Fn(&T, &T) -> T, UC: Fn(&U, &U) -> U, A: Fn(&T, &U, u32, u32) -> T> (
    t: &mut SegmentTree<T, U, TC, UC, A>, idx: i32
) -> Option<T> {
    if 0 <= idx && idx < t.len() as i32 {
        Some(t.get(idx as u32))
    } else {
        None
    }
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m] = read.usizes();
    let s = read.word_as_chars()
        .into_iter()
        .map(|ch| M::from(ch as u32 - 'a' as u32))
        .collect_vec();
    let mut p1 = BTreeSet::new();
    let mut p2 = BTreeSet::new();
    for i in 0..n {
        if Some(&s[i]) == s.get(i + 1) {
            p1.insert(i);
        }
        if Some(&s[i]) == s.get(i + 2) {
            p2.insert(i);
        }
    }
    let mut t = new_sum_tree(&s);
    let t = &mut t;
    for _ in 0..m {
        let qt = read.u32();
        match qt {
            1 => {
                let [l, r] = read.u32s().from1b();
                let x = read.u32();
                t.update(l..=r, &M::from(x));
                let l = l as i32;
                let r = r as i32;
                for (a, b) in [
                    (l - 1, l),
                    (r, r + 1),
                ] {
                    set_assign(&mut p1, a as usize, get_or(t, a).is_some() && get_or(t, a) == get_or(t, b));
                }
                for (a, b) in [
                    (l - 2, l),
                    (l - 1, l + 1),
                    (r - 1, r + 1),
                    (r, r + 2),
                ] {
                    set_assign(&mut p2, a as usize, get_or(t, a).is_some() && get_or(t, a) == get_or(t, b));
                }
            }
            2 => {
                let [l, r] = read.usizes().from1b();
                let mut c = false;
                if r >= l + 1 {
                    c |= p1.range(l..=(r - 1)).next().is_some();
                }
                if r >= l + 2 {
                    c |= p2.range(l..=(r - 2)).next().is_some();
                }
                emitln!(write, (!c).yesno());
            }
            _ => panic!()
        }
    }
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
    // use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        5
        12 8
        tedubcyyxopz
        2 2 5
        1 4 8 2
        2 1 7
        1 3 5 40
        1 9 11 3
        1 10 10 9
        2 4 10
        2 10 12
        10 4
        ubnxwwgzjt
        2 4 10
        2 10 10
        1 6 10 8
        2 7 7
        11 3
        hntcxfxyhtu
        1 4 6 1
        2 4 10
        1 4 10 21
        13 2
        yxhlmzfhqctir
        1 5 9 3
        1 8 13 15
        2 3
        bp
        1 1 2 15
        1 1 2 18
        1 2 2 1000000000
        "), "\
        YES
        NO
        NO
        YES
        NO
        YES
        YES
        YES
        ");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
