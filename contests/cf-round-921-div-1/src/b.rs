use std::collections::BTreeMap;

use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_zip_eq::IterutilsZipEq;
use contest_lib_rs::segment_tree::SegmentTree;


// Adds (a + bi) to the i-th element.
#[derive(Clone, Copy, Debug)]
struct Update {
    a: i64,
    b: i64,
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m, q] = read.usizes();
    let x = read.vec_u32(m).from1b();
    let v = read.vec_i64(m);
    let mut hrb = BTreeMap::from_iter(x.into_iter().zip_eq(v.into_iter()));

    let mut init = vec![0; n];
    for p in 0..n {
        let (&lp, &lv) = hrb.range(..=(p as u32)).next_back().unwrap();
        let (&rp, &rv) = hrb.range((p as u32)..).next().unwrap();
        init[p as usize] = lv * (rp as i64 - p as i64);
    }

    let mut tree = SegmentTree::new(
        &init,
        0,
        Update { a: 0, b: 0 },
        |v1, v2| {
            v1 + v2
        },
        |u1, u2| {
            Update {
                a: u1.a + u2.a,
                b: u1.b + u2.b,
            }
        },
        |v, u, l, r| {
            let l = l as i64;
            let r = r as i64;
            v + u.a * (r - l) + u.b * (r*(r-1) - l*(l-1)) / 2
        },
    );

    for _ in 0..q {
        let qt = read.u32();
        match qt {
            1 => {
                let p = read.u32().from1b();
                let v = read.i64();
                assert!(!hrb.contains_key(&p));
                let (&pl, &vl) = hrb.range(..=p).next_back().unwrap();
                let (&pr, &vr) = hrb.range(p..).next().unwrap();
                let ul = Update {
                    a: (p as i64 - pr as i64) * vl,
                    b: 0
                };
                let ur = Update {
                    a: (pr as i64) * (v - vl),
                    b: (vl - v),
                };
                hrb.insert(p, v);
                tree.update((pl + 1)..=p, &ul);
                tree.update((p + 1)..pr, &ur);
            }
            2 => {
                let [l, r] = read.u32s().from1b();
                emitln!(write, tree.get(l..=r));
            }
            _ => unreachable!()
        }
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
        assert_trimmed_eq!(&run_solver(solve, "\
        8 3 4
        1 3 8
        3 24 10
        2 2 5
        1 5 15
        2 5 5
        2 7 8
        "), "\
        171
        0
        15");
        assert_trimmed_eq!(&run_solver(solve, "\
        3 2 3
        1 3
        10 1000
        2 1 3
        1 2 100
        2 1 3
        "), "\
        10
        0
        ");
    }
}
