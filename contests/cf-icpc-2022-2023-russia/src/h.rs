use contest_lib_rs::bitset::Bitset;
use contest_lib_rs::directed_graph::DirectedGraph;
use contest_lib_rs::graph::{VertexId, Graph};
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::relax::RelaxMinMax;

fn update_p(g: &DirectedGraph<(), ()>, v: VertexId, pvec: &mut Vec<u32>, visited: &mut Bitset) {
    if visited.get(v.to_0_based() as usize) {
        return;
    }
    let mut newp = pvec[v];
    for (w, _) in g.edges_in(v) {
        update_p(g, w, pvec, visited);
        newp.relax_min(pvec[w] - 1);
    }
    pvec[v] = newp;
    visited.set(v.to_0_based() as usize, true);
}

fn dfs(g: &DirectedGraph<(), ()>, v: VertexId, visited: &mut Bitset) {
    if visited.get(v.to_0_based() as usize) {
        return;
    }
    for (w, _) in g.edges_out(v) {
        dfs(g, w, visited);
    }
    visited.set(v.to_0_based() as usize, true);
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m] = read.usizes();
    let mut pvec = read.vec_u32(n);
    let mut g = DirectedGraph::new();
    g.add_vertices(n);
    for _ in 0..m {
        let [a, b] = read.u32s();
        g.add_edge(VertexId::from_1_based(b), VertexId::from_1_based(a));
    }

    {
        let mut visited = Bitset::new(n);
        for v in g.vertex_ids() {
            update_p(&g, v, &mut pvec, &mut visited);
        }
    }

    let mut dep = Bitset::new(n);
    let mut x = vec![];
    let mut ans = vec![];
    for v in g.vertex_ids() {
        dep.fill(false);
        x.clear();
        dfs(&g, v, &mut dep);
        let mut num_dep = dep.count();
        for u in g.vertex_ids() {
            if !dep.get(u.to_0_based() as usize) {
                x.push(pvec[u]);
            }
        }
        x.sort_unstable();
        for i in (1..x.len()).rev() {
            let xx = x[i] - 1;
            x[i - 1].relax_min(xx);
        }
        let mut xp = 0;
        let mut t = 1;
        while num_dep > 0 {
            if t < *x.get(xp).unwrap_or(&u32::MAX) {
                t += 1;
                num_dep -= 1;
            } else {
                t += 1;
                xp += 1;
            }
        }
        ans.push(t - 1);
    }
    emitln!(write, ans);
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
        4 1
        2 3 2 4
        3 1
        "), "2 3 1 4");
        assert_trimmed_eq!(&run_solver(solve, "\
        3 0
        3 3 3
        "), "1 1 1");
        assert_trimmed_eq!(&run_solver(solve, "\
        5 3
        4 3 3 2 5
        3 1
        1 5
        4 2
        "), "4 2 1 1 5");
    }
}
