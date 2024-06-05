use contest_lib_rs::io::prelude::*;

fn super_or(a: u32, b: u32) -> u32 {
    let x = a ^ b;
    let mut y = x.next_power_of_two();
    if y == x {
        y *= 2;
    }
    let ret = a | (y - 1);
    assert_eq!(ret, b | (y - 1), "{a}, {b} => {x}, {y}");
    ret
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m] = read.u32s();
    emitln!(write, super_or(n.saturating_sub(m), n + m));
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
        9
        0 0
        0 1
        0 2
        1 0
        5 2
        10 1
        20 3
        1145 14
        19198 10
        "), "\
        0
        1
        3
        1
        7
        11
        23
        1279
        19455");
    }
}
