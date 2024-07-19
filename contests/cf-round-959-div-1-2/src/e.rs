use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let k = read.usize();
    let mut n = (0..k).map(|_| {
        let n = read.usize();
        _ = read.vec_usize(n - 1);
        n as u32
    }).collect_vec();
    n.sort_unstable_by_key(|x| std::cmp::Reverse(*x));
    let mut a = n[0];
    for v in &n[1..] {
        for i in (0..u32::BITS).rev() {
            let vi = (v >> i) & 1;
            let ai = (a >> i) & 1;
            match (vi, ai) {
                (0, _) => {}
                (1, 0) => {
                    a |= 1 << i;
                }
                (1, 1) => {
                    a |= (1 << i) - 1;
                }
                _ => unreachable!()
            }
        }
    }
    emitln!(write, a);
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
        assert_trimmed_eq!(&run_solver(solve, "\
        3
        1
        1

        2
        4
        1 2 2
        6
        1 1 3 1 3
        1
        10
        1 2 2 1 1 5 7 6 4"), "\
        1
        7
        10");
    }
}
