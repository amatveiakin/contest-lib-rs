use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, k, mut x] = read.u32s();
    if x < k - 1 {
        emitln!(write, -1);
        return;
    }
    if n < k {
        emitln!(write, -1);
        return;
    }
    if x == k {
        x -= 1;
    }
    let a = k * (k - 1) / 2 + (n - k) * x;
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
        9
        5 3 3
        4 7 5
        4 2 28
        12 10 6
        57 51 122
        200 1 200
        2 2 1
        3 2 1
        4 7 10
        "), "\
        7
        -1
        57
        -1
        2007
        39800
        1
        2
        -1");
    }
}
