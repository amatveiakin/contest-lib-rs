use contest_lib_rs::io::prelude::*;
use contest_lib_rs::sort_array::sort_array;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m] = read.usizes();
    let mut a = read.vec_u32(n);
    let b = read.vec_u32(m);
    let bor = b.iter().fold(0, |acc, &x| acc | x);
    let v1 = a.iter().fold(0, |acc, &x| acc ^ x);
    a.iter_mut().for_each(|x| *x |= bor);
    let v2 = a.iter().fold(0, |acc, &x| acc ^ x);
    let [min, max] = sort_array([v2, v1]);
    emitln!(write, min, max);
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
    let mut write = std::io::stdout().lock();
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
        2
        2 3
        0 1
        1 2 3
        3 1
        1 1 2
        1
        "), "\
        0 1
        2 3");
    }
}
