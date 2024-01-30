use contest_lib_rs::io::prelude::*;
use contest_lib_rs::sort_array::sort_array;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [mut a, mut b, r] = read.u64s();
    if a == b {
        emitln!(write, 0);
        return;
    }
    let mut first_diff = true;
    let mut fix = 0;
    for i in (0..u64::BITS).rev() {
        let v = 1 << i;
        let ai = a & v;
        let bi = b & v;
        if ai != bi {
            if first_diff {
                first_diff = false;
                [b, a] = sort_array([a, b]);
            } else {
                if ai > bi && fix | v <= r {
                    fix |= v;
                }
            }
        }
    }
    emitln!(write, (a ^ fix).abs_diff(b ^ fix));
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
        10
        4 6 0
        0 3 2
        9 6 10
        92 256 23
        165 839 201
        1 14 5
        2 7 2
        96549 34359 13851
        853686404475946 283666553522252166 127929199446003072
        735268590557942972 916721749674600979 895150420120690183
        "), "\
        2
        1
        1
        164
        542
        5
        3
        37102
        27934920819538516
        104449824168870225");
    }
}
