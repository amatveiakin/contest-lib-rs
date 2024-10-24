use std::collections::HashMap;

use contest_lib_rs::array_2d::DynArray2D;
use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::relax::Relax;

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m] = read.usizes();
    let k = read.u32();
    let s = (0..n).map(|_| read.word_as_chars()).collect_vec();

    let mut pp = HashMap::new();
    for (i, si) in s.iter().enumerate() {
        assert_eq!(si.len(), m);
        for (j, &ch) in si.iter().enumerate() {
            if ch == '.' {
                continue;
            }
            pp.entry((j, ch)).or_insert(vec![]).push(i);
        }
    }

    let mut mat: DynArray2D<u32> = DynArray2D::new(n, n);
    let mut colmax = vec![0; n];
    for y in 0..n {
        for j in 0..m {
            let ch = s[y][j];
            if ch == '.' {
                continue;
            }
            for &x in pp[&(j, ch)].iter() {
                if x >= y {
                    break;
                }
                mat[[x, y]] += 1;
                colmax[y].relax_max(mat[[x, y]]);
            }
        }
        if colmax[y] >= k {
            for x in (0..y).rev() {
                if mat[[x, y]] >= k {
                    emitln!(write, [x, y].to1b());
                    return;
                }
            }
            return;
        }
    }

    emitln!(write, -1);
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
        3 3 2
        BBC
        ..C
        .BC
        "), "1 3");
        assert_trimmed_eq!(&run_solver(solve, "\
        3 3 1
        BBC
        ..C
        .BC
        "), "1 2");
        assert_trimmed_eq!(&run_solver(solve, "\
        3 3 3
        BBC
        ..C
        .BC"), "-1");
        assert_trimmed_eq!(&run_solver(solve, "\
        4 12 2
        GOOD.LUCK.IN
        WINNING.ICPC
        ASIA.PACIFIC
        CHAMPIONSHIP
        "), "2 3");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
