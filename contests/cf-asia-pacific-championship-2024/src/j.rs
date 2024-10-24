use contest_lib_rs::base_one::Base;
use contest_lib_rs::dijkstra::{dijkstra_distances, dijkstra_path};
use contest_lib_rs::graph::Graph;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::iterutils_windows::IterutilsWindows;
use contest_lib_rs::relax::Relax;
use contest_lib_rs::sort_array::sort_array;
use contest_lib_rs::undirected_graph::UndirectedGraph;

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m] = read.usizes();
    let g = UndirectedGraph::from_read_edges_p(n, m, Base::ONE, read, |r| r.u64());
    let d1 = dijkstra_distances(&g, 0, |&p| p);
    let dn = dijkstra_distances(&g, n - 1, |&p| p);
    let Some(shp) = dijkstra_path(&g, 0, n - 1, |&p| p) else {
        emitln!(write, -1);
        return;
    };
    let shpv = shp.path.iter().copied().array_windows().map(|vs| sort_array(vs)).collect_set();
    let mut scnd = u64::MAX;
    for (a, b, w) in g.edges() {
        if !shpv.contains(&sort_array([a, b])) {
            if let (Some(x), Some(y)) = (d1.get(&a), dn.get(&b)) {
                scnd.relax_min(x + w + y);
            }
            if let (Some(x), Some(y)) = (d1.get(&b), dn.get(&a)) {
                scnd.relax_min(x + w + y);
            }
        }
    }
    if scnd == u64::MAX {
        emitln!(write, -1);
    } else {
        emitln!(write, shp.cost + scnd);
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
        3 2
        1 2 10
        1 3 5
        "), "30");
        assert_trimmed_eq!(&run_solver(solve, "\
        4 3
        1 2 10
        2 3 5
        3 4 2
        "), "-1");
        assert_trimmed_eq!(&run_solver(solve, "\
        4 4
        1 2 3
        2 4 2
        1 3 3
        3 4 4
        "), "12");
        assert_trimmed_eq!(&run_solver(solve, "\
        3 1
        1 2 1000"), "-1");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
