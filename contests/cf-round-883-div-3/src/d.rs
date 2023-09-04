use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let d = read.i32() as f64;
    let h = read.i32() as f64;

    let a = d * h / 2.0;
    let mut area = 0.0;
    let mut prev_y = None;
    for _ in 0..n {
        let y = read.i32() as f64;
        area += a;
        if let Some(prev_y) = prev_y {
            let dy: f64 = y - prev_y;
            if dy < h {
                area -= a * ((h - dy) / h).powi(2);
            }
        }
        prev_y = Some(y);
    }
    emitln!(write, area);
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
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        5
        3 4 2
        1 4 5
        1 5 1
        3
        4 6 6
        1 2 3 4
        2 1 200000
        1 200000
        2 4 3
        9 11
        "), "\
        11
        2.5
        34.5
        199999.9999975
        11.333333333333334");
    }
}
