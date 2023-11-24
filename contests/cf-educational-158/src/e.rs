use contest_lib_rs::graph::VertexId;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::relax::Relax;
use contest_lib_rs::tree::Tree;

fn dfs(v: VertexId, t: &Tree<(), ()>, a: &[i64], best: &mut Vec<[i64; 2]>) {
    for ch in t.children(v) {
        dfs(ch, t, a, best);
    }
    let mut b = [0, i64::MIN];

    let mut ch_best = t.children(v).map(|ch| best[ch][1]).collect_vec();
    ch_best.sort_unstable();
    let ch_best = ch_best;
    {
        // #parents == 1, #children == 0
        b[1].relax_max(a[v]);
    }
    if ch_best.len() >= 1 {
        // #parents == 1, #children == 1
        let ch_v = *ch_best.last().unwrap();
        b[1].relax_max(ch_v);
    }
    if ch_best.len() >= 2 {
        // #parents == 1, #children == 2+
        let mut ch_best = ch_best.clone();
        let mut ch_v = 0;
        ch_v += ch_best.pop().unwrap();
        ch_v += ch_best.pop().unwrap();
        ch_v += ch_best.iter().filter(|&&x| x > 0).sum::<i64>();
        b[1].relax_max(a[v] + ch_v);
    }

    {
        // already dead
        if t.children(v).len() >= 1 {
            let ch_v = t.children(v).map(|ch| best[ch][0]).max().unwrap();
            b[0].relax_max(ch_v);
        }
    }
    {
        // #parents == 0, #children == 0
        b[0].relax_max(a[v]);
    }
    if ch_best.len() >= 1 {
        // #parents == 0, #children == 1
        let ch_v = *ch_best.last().unwrap();
        b[0].relax_max(a[v] + ch_v);
    }
    if ch_best.len() >= 2 {
        // #parents == 0, #children == 2
        let mut ch_best = ch_best.clone();
        let mut ch_v = 0;
        ch_v += ch_best.pop().unwrap();
        ch_v += ch_best.pop().unwrap();
        b[0].relax_max(ch_v);
    }
    if ch_best.len() >= 3 {
        // #parents == 0, #children == 3+
        let mut ch_best = ch_best.clone();
        let mut ch_v = 0;
        ch_v += ch_best.pop().unwrap();
        ch_v += ch_best.pop().unwrap();
        ch_v += ch_best.pop().unwrap();
        ch_v += ch_best.iter().filter(|&&x| x > 0).sum::<i64>();
        b[0].relax_max(a[v] + ch_v);
    }

    best[v] = b;
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_i64(n);
    let t = Tree::from_read_edges(n, read).unwrap();
    let mut best = vec![[i64::MIN; 2]; n];
    dfs(t.root(), &t, &a, &mut best);
    emitln!(write, best[t.root()][0]);
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
        assert_trimmed_eq!(&run_solver(solve_case, "
        4
        -1 1 1 1
        1 2
        1 3
        1 4
        "), "2");

        assert_trimmed_eq!(&run_solver(solve_case, "
        1
        10
        "), "10");
        assert_trimmed_eq!(&run_solver(solve_case, "
        1
        -10
        "), "0");

        assert_trimmed_eq!(&run_solver(solve_case, "
        4
        -10 -1 10 10
        1 2
        2 3
        2 4
        "), "20");

        assert_trimmed_eq!(&run_solver(solve_case, "
        3
        -1 -1 -1
        1 2
        1 3
        "), "0");

        assert_trimmed_eq!(&run_solver(solve_case, "
        3
        -10 1 1
        1 2
        1 3
        "), "2");
        assert_trimmed_eq!(&run_solver(solve_case, "
        3
        1 -10 1
        1 2
        1 3
        "), "2");
        assert_trimmed_eq!(&run_solver(solve_case, "
        3
        1 -10 1
        1 2
        2 3
        "), "2");

        assert_trimmed_eq!(&run_solver(solve_case, "
        3
        10 1 1
        1 2
        1 3
        "), "11");
        assert_trimmed_eq!(&run_solver(solve_case, "
        3
        1 10 1
        1 2
        1 3
        "), "11");
        assert_trimmed_eq!(&run_solver(solve_case, "
        3
        1 10 1
        1 2
        2 3
        "), "11");

        assert_trimmed_eq!(&run_solver(solve_case, "
        4
        1 1 1 1
        1 2
        1 3
        1 4
        "), "4");

        assert_trimmed_eq!(&run_solver(solve_case, "
        5
        1 1 10 1 1
        1 2
        2 3
        3 4
        2 5
        "), "13");

        assert_trimmed_eq!(&run_solver(solve_case, "
        6
        1 1 -10 1 1 1
        1 2
        2 3
        3 4
        3 5
        2 6
        "), "4");

        assert_trimmed_eq!(&run_solver(solve_case, "
        4
        1 -10 1 1
        1 2
        2 3
        2 4
        "), "2");

        assert_trimmed_eq!(&run_solver(solve, "\
        3
        4
        1 -2 2 1
        1 2
        3 2
        2 4
        2
        -2 -5
        2 1
        7
        -2 4 -2 3 3 2 -1
        1 2
        2 3
        3 4
        3 5
        4 6
        4 7
        "), "\
        3
        0
        9");
    }
}
