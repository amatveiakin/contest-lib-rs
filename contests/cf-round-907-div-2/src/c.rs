use contest_lib_rs::counting_set::CountingSet;
use contest_lib_rs::int_ext::IntegerExtension;
use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let mut a = CountingSet::from_item_iter(read.vec_i64(n).into_iter());
    let mut c: i64 = 0;
    let mut s: i64 = 0;
    while !a.is_empty() {
        let x = a.pop_first().unwrap();
        if let Some(y) = a.pop_last() {
            let nc = c + x;
            if y <= nc {
                s += y + 1;
                c = 0;
                let d = nc - y;
                if d > 0 {
                    a.push(d);
                }
            } else {
                c = nc;
                a.push(y);
            }
        } else {
            let d = x - c;
            assert!(d >= 0);
            let x_self = if x > 1 {
                x.div_up(2) + 1
            } else {
                x
            };
            s += (c + d.div_up(2) + 1).min(c + x_self);
        }
    }
    assert!(a.is_empty());
    emitln!(write, s);
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
        assert_trimmed_eq!(&run_solver(solve_case, "1  1"), "1");
        assert_trimmed_eq!(&run_solver(solve_case, "1  2"), "2");
        assert_trimmed_eq!(&run_solver(solve_case, "1  3"), "3");
        assert_trimmed_eq!(&run_solver(solve_case, "1  4"), "3");
        assert_trimmed_eq!(&run_solver(solve_case, "1  9"), "6");
        assert_trimmed_eq!(&run_solver(solve_case, "4  1 2 1 1"), "4");
        assert_trimmed_eq!(&run_solver(solve_case, "2  1 6"), "5");
        assert_trimmed_eq!(&run_solver(solve, "\
        4
        4
        1 3 1 1
        4
        1 2 1 1
        6
        3 2 1 5 2 4
        2
        1 6
        "), "\
        4
        4
        11
        5");
    }
}
