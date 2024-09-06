// UNFINISHED

use contest_lib_rs::base_one::Base;
use contest_lib_rs::directed_graph::DirectedGraph;
use contest_lib_rs::graph::Graph;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;

fn num_combinations_f64(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    let mut ret = 1.0;
    for i in 0..k {
        ret *= (n - i) as f64;
        ret /= (i + 1) as f64;
    }
    ret
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m] = read.usizes();
    let g = DirectedGraph::from_read_edges(n, m, Base::ONE, read);
    let mut probs = vec![1.0; n];
    for u in (0..(n - 1)).rev() {
        let mut next = g.edges_out(u).map(|(v, _)| v).collect_vec();
        next.sort();
        next.reverse();
        let n_next = next.len();
        let mut pu = 0.0;
        let mut p_already_gone_prev_iter = 0.0;
        for iter in 0..((n_next + 1) / 2) {
            let mut p_already_gone_next_iter = 0.0;
            for (iv, &v) in next.iter().enumerate() {
                if iv < iter {
                    continue;
                }
                let they_discarded_before = iv - iter;
                if they_discarded_before > iter {
                    continue;
                }
                let p_they_discarded_ok =
                    num_combinations_f64(n_next - iv - 1, iter - they_discarded_before)
                    / num_combinations_f64(n_next - iter, iter);
                let p_match = 1. / ((n_next - iter * 2) as f64);
                let p_went_next = (1.0 - p_already_gone_prev_iter) * p_they_discarded_ok * p_match;
                pu += p_went_next * probs[v];
                p_already_gone_next_iter += p_went_next;
            }
            p_already_gone_prev_iter = p_already_gone_next_iter;
        }
        probs[u] = pu;
    }
    emitln!(write, probs[0]);
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
        assert_trimmed_eq!(&run_solver(solve_case, "\
        4 5
        1 4
        1 2
        2 4
        1 3
        3 4
        "), "1");

        assert_trimmed_eq!(&run_solver(solve_case, "\
        4 4
        1 2
        2 4
        1 3
        3 4
        "), "0.5");

        assert_trimmed_eq!(&run_solver(solve_case, "\
        5 4
        1 2
        2 3
        3 4
        4 5
        "), "1");

        assert_trimmed_eq!(&run_solver(solve_case, "\
        5 6
        1 2
        2 5
        1 3
        3 5
        1 4
        4 5
        "), "1");

        assert_trimmed_eq!(&run_solver(solve_case, "\
        4 6
        1 2
        1 3
        1 4
        2 3
        2 4
        3 4
        "), "0.8333333333333335");

        // assert_trimmed_eq!(&run_solver(solve, "\
        // 3
        // 3 2
        // 1 2
        // 1 3
        // 7 8
        // 1 2
        // 1 3
        // 1 4
        // 1 5
        // 2 6
        // 3 6
        // 4 6
        // 6 7
        // 10 20
        // 1 2
        // 1 3
        // 1 4
        // 1 5
        // 1 6
        // 2 6
        // 2 7
        // 2 8
        // 2 9
        // 3 4
        // 3 7
        // 3 8
        // 3 10
        // 4 6
        // 4 8
        // 4 10
        // 6 10
        // 7 8
        // 7 9
        // 7 10
        // "), "\
        // 0.500000000000
        // 0.625000000000
        // 0.491666666667");
    }
}
