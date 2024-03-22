use contest_lib_rs::io::prelude::*;
use contest_lib_rs::mod_ring::ModNumber;
use contest_lib_rs::relax::Relax;

type ModNum = ModNumber<1000000007>;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let k = read.u32();
    let a = read.vec_i64(n);

    let mut s = 0;
    let mut smax = 0;
    for x in a.iter() {
        s += x;
        s.relax_max(0);
        smax.relax_max(s);
    }
    let ans = ModNum::from(a.iter().sum::<i64>())
        + ModNum::from(smax) * (ModNum::from(2).pow(k) - ModNum::from(1));
    emitln!(write, ans)
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
        12
        2 2
        -4 -7
        3 3
        2 2 8
        1 7
        7
        5 1
        4 -2 8 -12 9
        7 4
        8 14 -9 6 0 -1 3
        7 100
        5 3 -8 12 -5 -9 3
        6 1000
        -1000000000 -1000000000 -1000000000 -1000000000 -1000000000 -1000000000
        2 1
        1000000000 8
        5 4
        0 0 0 0 0
        6 10
        48973 757292 58277 -38574 27475 999984
        7 1
        -1000 1000 -1000 1000 -1000 1000 -1000
        10 10050
        408293874 -3498597 7374783 295774930 -48574034 26623784 498754833 -294875830 283045804 85938045
        "), "\
        999999996
        96
        896
        17
        351
        716455332
        42
        2
        0
        897909241
        0
        416571966");
    }
}
