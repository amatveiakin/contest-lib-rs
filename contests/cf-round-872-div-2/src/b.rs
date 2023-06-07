use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.i32();
    let m = read.i32();
    let mut b = read.vec_i32((n * m) as usize);
    let answer1;
    let answer2;
    {
        b.sort();
        let max = b.last().unwrap();
        let min1 = b[0];
        let min2 = b[1];
        let (n, m) = if n < m { (n, m) } else { (m, n) };
        answer1 = (max - min2) * (n - 1) + (max - min1) * n * (m - 1);
    }
    {
        b.reverse();
        let min = b.last().unwrap();
        let max1 = b[0];
        let max2 = b[1];
        let (n, m) = if n < m { (n, m) } else { (m, n) };
        answer2 = (max2 - min) * (n - 1) + (max1 - min) * n * (m - 1);
    }
    let answer = std::cmp::max(answer1, answer2);
    emitln!(write, answer);
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let t = read.usize();
    for _ in 0..t {
        solve_case(read, write);
    }
}

fn main() {
    let mut read = io::Reader::new(std::io::stdin().lock());
    let mut write = std::io::stdout().lock();
    solve(&mut read, &mut write);
}


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use pretty_assertions::assert_eq;
    use contest_lib_rs::{solution_testing::run_solver, assert_trimmed_eq};

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        5
        2 2
        1 3 1 4
        2 2
        -1 -1 -1 -1
        2 3
        7 8 9 -3 10 8
        3 2
        4 8 -3 0 -7 1
        4 3
        -32030 59554 16854 -85927 68060 -64460 -79547 90932 85063 82703 -12001 38762
        "), "\
        9
        0
        64
        71
        1933711");
    }
}
