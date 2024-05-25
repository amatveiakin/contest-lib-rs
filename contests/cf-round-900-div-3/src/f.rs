use contest_lib_rs::bool_ext::BoolExtension;
use contest_lib_rs::factored_num::FactoredNum;
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n_orig = read.u32();
    let q = read.usize();

    let n_orig = FactoredNum::from(n_orig);
    let mut n = n_orig.clone();
    for _ in 0..q {
        let qt = read.u32();
        match qt {
            1 => {
                let x = FactoredNum::from(read.u32());
                n *= x;
                let d = FactoredNum::from(n.num_divisors());
                emitln!(write, n.divisible_by(&d).YESNO());
            }
            2 => {
                n = n_orig.clone();
            }
            _ => unreachable!()
        }
    }
    emitln!(write);
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
        7
        1 5
        1 1
        1 2
        2
        1 8
        1 9
        20 4
        1 3
        2
        1 7
        1 12
        16 10
        1 6
        1 6
        1 10
        1 9
        1 1
        1 9
        1 7
        1 3
        1 2
        1 10
        9 1
        1 3
        8 1
        1 2
        8 3
        1 5
        1 8
        1 10
        11 5
        1 8
        1 2
        1 1
        1 3
        1 1
        "), "\
        YES
        YES
        YES
        YES

        YES
        NO
        YES

        YES
        NO
        YES
        YES
        YES
        NO
        YES
        NO
        YES
        YES

        NO

        NO

        YES
        NO
        NO

        YES
        NO
        NO
        NO
        NO
        ");
    }
}
