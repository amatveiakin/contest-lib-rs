use contest_lib_rs::counting_set::CountingSet;
use contest_lib_rs::factors::factors;
use contest_lib_rs::io::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
struct MultNum(CountingSet<u32>);

impl MultNum {
    fn from(x: u32) -> Self {
        let mut s = CountingSet::new();
        for (p, c) in factors(x) {
            s.push_multiple(p, c as usize);
        }
        Self(s)
    }

    fn mul(mut self, rhs: &Self) -> Self {
        for (&p, c) in rhs.0.iter_groups() {
            self.0.push_multiple(p, c);
        }
        self
    }

    fn num_div(&self) -> u32 {
        self.0.iter_groups().map(|(_, c)| c as u32 + 1).product()
    }

    fn divisible_by(&self, rhs: &Self) -> bool {
        self.0.is_superset(&rhs.0)
    }
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n_orig = read.u32();
    let q = read.usize();

    let n_orig = MultNum::from(n_orig);
    let mut n = n_orig.clone();
    for _ in 0..q {
        let qt = read.u32();
        match qt {
            1 => {
                let x = MultNum::from(read.u32());
                n = n.mul(&x);
                let d = MultNum::from(n.num_div());
                if n.divisible_by(&d) {
                    emitln!(write, "YES");
                } else {
                    emitln!(write, "NO");
                }
            }
            2 => {
                n = n_orig.clone();
            }
            _ => unreachable!()
        }
    }
    emitln!(write, "");
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
