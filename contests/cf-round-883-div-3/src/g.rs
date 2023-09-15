use contest_lib_rs::dijkstra::dijkstra_path;
use contest_lib_rs::directed_graph::DirectedGraph;
use contest_lib_rs::graph::Graph;
use contest_lib_rs::{io, emitln};
use contest_lib_rs::relax::RelaxMinMax;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let m = read.usize();
    let rudolf = usize::from_str_radix(&read.word(), 2).unwrap();
    let mut g: DirectedGraph<(), u64> = DirectedGraph::new();
    g.add_vertices(1 << n);
    for _ in 0..m {
        let d = read.u64();
        let removes = usize::from_str_radix(&read.word(), 2).unwrap();
        let adds = usize::from_str_radix(&read.word(), 2).unwrap();
        for v in 0..(1 << n) {
            let u = v & !removes | adds;
            if let Some(old_d) = g.edge_mut(v, u) {
                old_d.relax_min(d);
            } else {
                g.add_edge_p(v, u, d);
            }
        }
    }
    let rudolf = rudolf;
    let healthy = 0;
    let time = dijkstra_path(&g, rudolf, healthy, |&x| x).map_or(-1, |path| path.cost as i64);
    emitln!(write, time);
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let t = read.usize();
    for _ in 0..t {
        solve_case(read, write);
    }
}

fn main() {
    let mut read = io::Reader::new(std::io::stdin().lock());
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
        4
        5 4
        10011
        3
        10000
        00110
        3
        00101
        00000
        3
        01010
        00100
        5
        11010
        00100
        4 1
        0000
        10
        1011
        0100
        2 2
        11
        2
        10
        01
        3
        01
        10
        2 3
        11
        3
        01
        10
        3
        10
        00
        4
        10
        01
        "), "\
        8
        0
        -1
        6");
    }
}
