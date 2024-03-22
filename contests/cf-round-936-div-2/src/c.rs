use contest_lib_rs::io::prelude::*;
use contest_lib_rs::tree::Tree;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let k = read.u32();
    let t = Tree::from_read_edges(n, read).unwrap();
    let mut l = 1;
    let mut r = n;
    while l < r {
        let x = (l + r + 1) / 2;
        let data = t.compute_recursively(|ch_data, _| {
            let mut nc = 0;
            let mut nv = 1;
            for (ch_nc, ch_nv) in ch_data {
                nc += ch_nc;
                nv += ch_nv;
            }
            if nv >= x {
                nc += 1;
                nv = 0;
            }
            (nc, nv)
        });
        let (nc, _) = data[t.root()];
        let ok = nc >= k + 1;
        if ok {
            l = x;
        } else {
            r = x - 1;
        }
    }
    emitln!(write, l);
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
        6
        5 1
        1 2
        1 3
        3 4
        3 5
        2 1
        1 2
        6 1
        1 2
        2 3
        3 4
        4 5
        5 6
        3 1
        1 2
        1 3
        8 2
        1 2
        1 3
        2 4
        2 5
        3 6
        3 7
        3 8
        6 2
        1 2
        2 3
        1 4
        4 5
        5 6
        "), "\
        2
        1
        3
        1
        1
        2");
    }
}
