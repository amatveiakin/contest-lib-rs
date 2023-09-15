// UNFINISHED

use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::directed_graph::DirectedGraph;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::topological_sort::topological_sort;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, mut k] = read.usizes();
    let b = read.vec_i32(n).from1b();
    assert!(b.iter().all(|&x| x >= 0 && x < n as i32));
    let mut s = b.clone();
    let mut r = (0..n).collect::<Vec<_>>();

    if k == 1 {
        for i in 0..n {
            if b[i] != i as i32 {
                emitln!(write, "NO");
                return;
            }
        }
        emitln!(write, "YES");
        return;
    }

    while k > 0 {
        if k & 1 == 1 {
            let mut nr = vec![0; n];
            for i in 0..n {
                nr[i] = r[s[i] as usize];
            }
            r = nr;
        }
        let mut ns = vec![0; n];
        for i in 0..n {
            ns[i] = s[s[i] as usize];
        }
        s = ns;
        k >>= 1;
    }

    let mut g = DirectedGraph::new();
    g.add_vertices(n);
    for i in 0..n {
        if r[i] != i {
            g.add_edge(i, r[i]);
        }
    }
    let ans = topological_sort(&g).is_some();
    emitln!(write, if ans { "YES" } else { "NO" });
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
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        6
        5 3
        2 3 5 3 4
        4 2
        2 4 3 1
        1 1
        1
        3 1
        1 2 3
        5 3
        5 4 3 2 1
        6 1
        1 2 3 1 5 6
        "), "\
        YES
        NO
        YES
        YES
        NO
        NO");
    }
}
