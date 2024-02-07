use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::bfs::bfs_path;
use contest_lib_rs::bridges::find_bridges;
use contest_lib_rs::graph::Graph;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::undirected_graph::{UndirectedEdgeId, UndirectedGraph};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m] = read.usizes();
    let mut g = UndirectedGraph::from_read_edges_p(n, m, read, |read| read.u32());
    let br = find_bridges(&g);
    let mut minw = u32::MAX;
    let mut mine = None;
    for (u, v, &w) in g.edges() {
        if w < minw && !br.contains(&UndirectedEdgeId::new(u, v)) {
            minw = w;
            mine = Some((u, v));
        }
    }
    let (u, v) = mine.unwrap();
    g.remove_edge(u, v);
    let cycle = bfs_path(&g, u, v).unwrap();
    emitln!(write, minw, cycle.len());
    emitln!(write, cycle.to1b());
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
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 5
        // 6 6
        // 1 2 1
        // 2 3 1
        // 3 1 1
        // 4 5 1
        // 5 6 1
        // 6 4 1
        // 6 6
        // 1 2 10
        // 2 3 8
        // 3 1 5
        // 4 5 100
        // 5 6 40
        // 6 4 3
        // 6 15
        // 1 2 4
        // 5 2 8
        // 6 1 7
        // 6 3 10
        // 6 5 1
        // 3 2 8
        // 4 3 4
        // 5 3 6
        // 2 6 6
        // 5 4 5
        // 4 1 3
        // 6 4 5
        // 4 2 1
        // 3 1 7
        // 1 5 5
        // 4 6
        // 2 3 2
        // 1 3 10
        // 1 4 1
        // 3 4 7
        // 2 4 5
        // 1 2 2
        // 4 5
        // 2 1 10
        // 3 1 3
        // 4 2 6
        // 1 4 7
        // 2 3 3
        // "), "\
        // 1 3
        // 1 2 3
        // 3 3
        // 6 4 5
        // 1 5
        // 4 2 1 6 3
        // 1 4
        // 1 4 3 2
        // 3 3
        // 2 3 1 ");
    }
}
