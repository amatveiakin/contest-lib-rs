use contest_lib_rs::graph::Graph;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::undirected_graph::UndirectedGraph;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m] = read.usizes();
    let g = UndirectedGraph::from_read_edges_p(n, m, read, |r| r.u32());

    let mut gg = UndirectedGraph::new();
    let mut gempty = UndirectedGraph::new();
    gg.add_vertices(n);
    gempty.add_vertices(n);
    for (u, v, &p) in g.edges() {
        if p == 1 {
            gg.add_edge(u, v);
        } else {
            gempty.add_edge(u, v);
        }
    }

    let mut vodd = gg.vertex_ids().filter(|&v| gg.degree(v) % 2 == 1).collect_set();
    while vodd.len() > 0 {
        let u = *vodd.iter().next().unwrap();
        assert!(vodd.remove(&u));
        let mut v = u;
        while u == v || gg.degree(v) % 2 == 0 {
            let (w, _) = gempty.edges_out(v).next().unwrap();
            gempty.remove_edge(v, w);
            v = w;
        }
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
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
