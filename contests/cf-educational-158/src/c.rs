use contest_lib_rs::counting_set::CountingSet;
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let mut a = CountingSet::from_item_iter(read.vec_u32(n).into_iter());
    let mut xs = vec![];
    while a.num_groups() > 1 {
        let m = *a.last().unwrap();
        let x = 1 - m % 2;
        let mut b = CountingSet::new();
        for (&v, c) in a.group_iter() {
            b.push_multiple((v + x) / 2, c);
        }
        a = b;
        xs.push(x);
    }
    emitln!(write, xs.len());
    if 0 < xs.len() && xs.len() <= n {
        emitln!(write, xs);
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
        4
        1
        10
        2
        4 6
        6
        2 1 2 1 2 1
        2
        0 32
        "), "\
        0
        2
        1 0
        1
        1
        6");
        assert_trimmed_eq!(&run_solver(solve_case, "2  8 5"), "2\n1 1");
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 4
        // 1
        // 10
        // 2
        // 4 6
        // 6
        // 2 1 2 1 2 1
        // 2
        // 0 32
        // "), "\
        // 0
        // 2
        // 2 5
        // 1
        // 1
        // 6");
    }
}
