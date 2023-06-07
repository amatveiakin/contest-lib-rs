use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let s = read.word();
    let mut balance: i32 = 0;
    let mut colors = vec![];
    let balance_tmpl: i32 = match s.chars().next().unwrap() {
        '(' => 1,
        ')' => -1,
        _ => unreachable!(),
    };
    for c in s.chars() {
        let old_balance = balance;
        match c {
            '(' => balance += 1,
            ')' => balance -= 1,
            _ => unreachable!(),
        }
        if (balance + old_balance).signum() == balance_tmpl.signum() {
            colors.push(1);
        } else {
            colors.push(2);
        }
    }
    if balance != 0 {
        emitln!(write, -1);
    } else {
        emitln!(write, colors.iter().max().unwrap());
        emitln!(write, colors);
    }
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
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 4
        // 8
        // ((())))(
        // 4
        // (())
        // 4
        // ))((
        // 3
        // (()
        // "), "\
        // 2
        // 2 2 2 1 2 2 2 1
        // 1
        // 1 1 1 1
        // 1
        // 1 1 1 1
        // -1");
        assert_trimmed_eq!(&run_solver(solve, "\
        4
        8
        ((())))(
        4
        (())
        4
        ))((
        3
        (()
        "), "\
        2
        1 1 1 1 1 1 2 2
        1
        1 1 1 1
        1
        1 1 1 1
        -1");
    }
}
