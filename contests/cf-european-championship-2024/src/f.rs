use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;

const Q: u32 = 8901;

struct User {
    id: usize,
    acts: Vec<u32>,
    acts_set: Set,
}

#[derive(Clone, Default)]
struct Segment {
    vals: Vec<u32>,
    hash: u64,
}

struct Set {
    segs: Vec<Segment>,
}

impl Set {
    fn new(vals_sorted: &[u32]) -> Self {
        let mut segs = vec![];
        for v in vals_sorted {
            let i_seg = (*v / Q) as usize;
            let seg_v = *v % Q;
            if segs.len() < i_seg + 1 {
                segs.resize(i_seg + 1, Segment::default());
            }
            segs[i_seg].vals.push(seg_v);
        }
        for seg in segs.iter_mut() {
            seg.vals.shrink_to_fit();
            seg.hash = calculate_hash(&seg.vals);
        }
        Set { segs }
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn contains_values(haystack: &[u32], needle: &[u32]) -> bool {
    let mut i = 0;
    for &n in needle {
        while i < haystack.len() && haystack[i] < n {
            i += 1;
        }
        if i < haystack.len() && haystack[i] == n {
            // found
        } else {
            return false;
        }
    }
    true
}

fn contains_set(haystack: &Set, needle: &Set) -> bool {
    if needle.segs.len() > haystack.segs.len() {
        return false;
    }
    for i in 0..needle.segs.len() {
        if haystack.segs[i].hash == needle.segs[i].hash {
            continue;
        }
        if !contains_values(&haystack.segs[i].vals, &needle.segs[i].vals) {
            return false;
        }
    }
    true
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m] = read.usizes();
    let mut ks = (0..n)
        .map(|id| {
            let s = read.usize();
            let mut acts = read.vec_u32(s).from1b();
            acts.shrink_to_fit();
            acts.sort_unstable();
            let acts_set = Set::new(&mut acts);
            User { id, acts, acts_set }
        })
        .collect_vec();
    ks.sort_by_cached_key(|k| (k.acts.len(), k.acts.clone()));

    let mut grs = vec![vec![]; m];
    for (user, k) in ks.iter().enumerate() {
        for &a in k.acts.iter() {
            grs[a as usize].push(user as u32);
        }
    }

    let mut contains_cache = HashSet::new();
    for gr in grs {
        let mut pu: Option<u32> = None;
        for nu in gr {
            if let Some(pu) = pu {
                if !contains_cache.contains(&(pu, nu)) {
                    if !contains_set(&ks[nu as usize].acts_set, &ks[pu as usize].acts_set) {
                        emitln!(write, "YES");
                        emitln!(write, [ks[pu as usize].id, ks[nu as usize].id].to1b());
                        return;
                    }
                    contains_cache.insert((pu, nu));
                }
            }
            pu = Some(nu);
        }
    }
    emitln!(write, "NO");
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
    use contest_lib_rs::testing::solution_testing::prelude::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve,"\
        3 5
        3 1 2 4
        5 1 2 3 4 5
        2 1 5"), "\
        YES
        3 1"
        );
        assert_trimmed_eq!(&run_solver(solve, "\
        3 3
        1 1
        1 2
        3 2 3 1"), "NO");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
