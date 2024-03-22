use contest_lib_rs::io::prelude::*;
use contest_lib_rs::relax::Relax;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let x = read.u32();
    let a = read.vec_u32(n);
    let mut candidates = vec![x];
    for i in 0..30 {
        if x & (1 << i) != 0 {
            candidates.push(x & !(1 << i) | ((1 << i) - 1));
        }
    }
    let mut kmax = -1;
    for x in candidates {
        let mut k = 0;
        let mut axor = 0;
        for &ai in &a {
            axor ^= ai;
            if axor | x == x {
                k += 1;
                axor = 0;
            }
        }
        if axor == 0 {
            kmax.relax_max(k);
        }
    }
    emitln!(write, kmax);
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
        8
        3 1
        1 2 3
        2 2
        1 1
        2 2
        1 3
        2 3
        0 0
        3 2
        0 0 1
        4 2
        1 3 3 7
        2 2
        2 3
        5 0
        0 1 2 2 1
        "), "\
        2
        2
        1
        2
        3
        -1
        1
        2");
    }
}
