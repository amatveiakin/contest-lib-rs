use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::disjoint_set::DisjointSet;
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u32(n);

    let mut comp = DisjointSet::new(n);
    let mut ops = vec![];
    'outer: for x in (1..n).rev() {
        let mut modgr = vec![None; x];
        for (i, &v) in a.iter().enumerate() {
            let vx = v as usize % x;
            if let Some(j) = modgr[vx] {
                if comp.unite(i, j) {
                    ops.push((i, j));
                    continue 'outer;
                }
            } else {
                modgr[vx] = Some(i);
            }
        }
        panic!();
    }

    emitln!(write, "YES");
    for &(i, j) in ops.iter().rev() {
        emitln!(write, i.to1b(), j.to1b());
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
    use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 8
        // 2
        // 1 4
        // 4
        // 99 7 1 13
        // 5
        // 10 2 31 44 73
        // 5
        // 87 6 81 44 32
        // 5
        // 62 35 33 79 16
        // 5
        // 6 51 31 69 42
        // 5
        // 52 63 25 21 5
        // 12
        // 33 40 3 11 31 43 37 8 50 5 12 22"), "\
        // YES
        // 2 1
        // YES
        // 4 1
        // 2 1
        // 3 2
        // YES
        // 5 1
        // 4 1
        // 3 1
        // 2 1
        // YES
        // 4 1
        // 3 1
        // 2 1
        // 5 4
        // YES
        // 3 1
        // 5 1
        // 2 1
        // 4 2
        // YES
        // 4 1
        // 5 1
        // 2 1
        // 3 2
        // YES
        // 2 1
        // 5 2
        // 3 1
        // 4 3
        // YES
        // 9 1
        // 12 9
        // 11 1
        // 10 1
        // 6 1
        // 7 6
        // 2 1
        // 8 2
        // 5 2
        // 3 1
        // 4 1");
    }
}
