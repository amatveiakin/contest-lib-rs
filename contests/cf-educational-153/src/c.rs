use contest_lib_rs::io::prelude::*;
use contest_lib_rs::relax::RelaxMinMax;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let p = read.vec_i32(n);
    let mut min = i32::MAX;
    let mut min_loss = i32::MAX;
    let mut num_lucky = 0;
    for x in p {
        let has_any = min < x;
        let has_loss = min_loss < x;
        let victory = has_loss || !has_any;
        if !victory {
            num_lucky += 1;
            min_loss.relax_min(x);
        }
        min.relax_min(x);
    }
    emitln!(write, num_lucky);
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
4
3
2 1 3
2
2 1
3
1 2 3
4
2 1 4 3"), "\
1
0
1
2");
    }
}
