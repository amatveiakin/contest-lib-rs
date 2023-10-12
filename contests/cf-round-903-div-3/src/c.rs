use contest_lib_rs::array_2d::{CharArray2DReading, Array2D};
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.char_array2d(n, n).map(|c| c as u32);
    let mut ans = 0;
    for i in 0..(n / 2) {
        for j in 0..(n / 2) {
            let q = [
                a[[i, j]],
                a[[j, n - 1 - i]],
                a[[n - 1 - j, i]],
                a[[n - 1 - i, n - 1 - j]],
            ];
            let s = q.iter().sum::<u32>();
            let m = *q.iter().max().unwrap();
            ans += q.len() as u32 * m - s;
        }
    }
    emitln!(write, ans);
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
        5
        4
        abba
        bcbb
        bccb
        abba
        2
        ab
        ba
        6
        codefo
        rcesco
        deforc
        escode
        forces
        codefo
        4
        baaa
        abba
        baba
        baab
        4
        bbaa
        abba
        aaba
        abba
        "), "\
        1
        2
        181
        5
        9");
    }
}
