use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let s = read.word();
    if s == "()" {
        emitln!(write, "NO");
    } else if s.contains("((") || s.contains("))") {
        let a = "()".repeat(s.len());
        emitln!(write, "YES");
        emitln!(write, a);
    } else {
        let a = "(".repeat(s.len()) + &")".repeat(s.len());
        emitln!(write, "YES");
        emitln!(write, a);
    }
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
    use contest_lib_rs::{solution_testing::run_solver, assert_trimmed_eq};

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
4
)(
(()
()
))()"), "\
YES
(())
YES
()()()
NO
YES
()()()()");
    }
}
