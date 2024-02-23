use contest_lib_rs::io::prelude::*;
use contest_lib_rs::mod_ring::ModNumber;
use contest_lib_rs::tree::Tree;

type M = ModNumber<998244353>;

#[derive(Clone, Copy, Debug)]
struct V {
    len: M,
    pr: M,
    tr: M,
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let t = Tree::from_read_edges(n, read).unwrap();
    let v = t.compute_recursively(|ch: &[&V], _| {
        let mut len = M::from(1);
        let mut pr = M::from(1);
        let mut tr = M::from(1);
        for u in ch {
            pr *= u.pr;
            tr *= u.pr;
        }
        pr += M::from(1);
        tr += M::from(1);
        for u in ch {
            len += u.len;
            // tr += (u.tr - M::from(1)) - (u.pr - M::from(1));
            // tr += u.pr - M::from(1);
            tr += u.tr - M::from(1);
        }
        V { len, pr, tr }
    });
    emitln!(write, v[t.root()].tr);
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
    // use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve_case, "3  1 2  2 3"), "7");
        assert_trimmed_eq!(&run_solver(solve_case, "3  1 2  1 3"), "7");
        assert_trimmed_eq!(&run_solver(solve_case, "4  1 2  2 3  3 4"), "11");
        assert_trimmed_eq!(&run_solver(solve, "\
        4
        3
        1 3
        3 2
        4
        3 4
        2 3
        3 1
        5
        1 2
        3 4
        5 1
        2 3
        4
        1 2
        2 3
        3 4
        "), "\
        7
        12
        16
        11");
    }
}
