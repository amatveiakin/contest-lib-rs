use contest_lib_rs::io;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.u32();
    if n == 1 {
        writeln!(write, "a").unwrap();
        return;
    }
    if n == 2 {
        writeln!(write, "ab").unwrap();
        return;
    }
    for k in 2..30 {
        if n % k != 0 {
            let s = (0..n).map(|i| (b'a' + (i % k) as u8) as char).collect::<String>();
            writeln!(write, "{}", s).unwrap();
            return;
        }
    }
    unreachable!();
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
        4
        4
        2
        1
        6
        "), "\
        abca
        ab
        a
        abcdab");
    }
}
