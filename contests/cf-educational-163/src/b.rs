use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let mut a = read.vec_u32(n);
    let mut last = a.pop().unwrap();
    while let Some(v) = a.pop() {
        if v > last {
            if v < 10 {
                emitln!(write, "NO");
                return;
            } else {
                a.push(v / 10);
                a.push(v % 10);
            }
        } else {
            last = v;
        }
    }
    emitln!(write, "YES");
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
        3
        4
        12 3 45 67
        3
        12 28 5
        2
        0 0
        "), "\
        YES
        NO
        YES");
    }
}
