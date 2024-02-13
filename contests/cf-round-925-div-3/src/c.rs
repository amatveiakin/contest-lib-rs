use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u32(n);
    let first = *a.first().unwrap();
    let last = *a.last().unwrap();
    if a.iter().all(|&x| x == first) {
        emitln!(write, 0);
        return;
    }
    let ans1 =
        a.iter().rposition(|&x| x != first).unwrap() -
        a.iter().position(|&x| x != first).unwrap() + 1;
    let ans2 =
        a.iter().rposition(|&x| x != last).unwrap() -
        a.iter().position(|&x| x != last).unwrap() + 1;
    let ans = ans1.min(ans2);
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
        8
        6
        1 2 3 4 5 1
        7
        1 1 1 1 1 1 1
        8
        8 8 8 1 2 8 8 8
        1
        1
        2
        1 2
        3
        1 2 3
        7
        4 3 2 7 1 1 3
        9
        9 9 2 9 2 5 5 5 3
        "), "\
        4
        0
        2
        0
        1
        2
        6
        7");
    }
}
