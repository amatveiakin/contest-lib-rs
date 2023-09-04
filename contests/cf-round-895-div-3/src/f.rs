use contest_lib_rs::directed_graph::DirectedGraph;
use contest_lib_rs::graph::{Graph, VertexId};
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::topological_sort::topological_sort;

#[derive(Clone, Copy, Default, Debug)]
struct VertexState {
    visited: bool,
    active: bool,
}

fn visit(graph: &DirectedGraph<(), ()>, v: VertexId, vertex_state: &mut Vec<VertexState>)
    -> Option<Vec<VertexId>>
{
    if vertex_state[v].visited {
        return None;
    }
    if vertex_state[v].active {
        return Some(vec![v]);
    }
    vertex_state[v].active = true;
    for e in graph.edges_out(v) {
        if let Some(mut cycle) = visit(graph, e.other, vertex_state) {
            cycle.push(v);
            vertex_state[v].active = false;
            vertex_state[v].visited = true;
            return Some(cycle);
        }
    }
    vertex_state[v].active = false;
    vertex_state[v].visited = true;
    None
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u32(n);
    let c = read.vec_u32(n);

    let mut g = DirectedGraph::new();
    g.add_vertices(n);
    for i in 0..n {
        g.add_edge(VertexId::from_0_based(i), VertexId::from_1_based(a[i]));
    }

    let mut vertex_state = vec![VertexState::default(); n];
    for v in g.vertex_ids() {
        if !vertex_state[v].visited {
            if let Some(stack) = visit(&g, v, &mut vertex_state) {
                let p = stack[1..].iter().position(|&w| w == stack[0]).unwrap();
                let cycle = &stack[..=p];
                let min = *cycle.iter().min_by_key(|&&w| c[w]).unwrap();
                assert!(g.remove_edge(min, VertexId::from_1_based(a[min])).is_some());
            }
        }
    }

    let order = topological_sort(&g).unwrap().into_iter().map(|v| v.to_1_based()).collect::<Vec<_>>();
    emitln!(write, order);
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
        2
        2 1
        100 10
        "), "\
        1 2");
        assert_trimmed_eq!(&run_solver(solve_case, "\
        2
        2 1
        10 100
        "), "\
        2 1");

        // assert_trimmed_eq!(&run_solver(solve, "\
        // 8
        // 3
        // 2 3 2
        // 6 6 1
        // 8
        // 2 1 4 3 6 5 8 7
        // 1 2 1 2 2 1 2 1
        // 5
        // 2 1 1 1 1
        // 9 8 1 1 1
        // 2
        // 2 1
        // 1000000000 999999999
        // 7
        // 2 3 2 6 4 4 3
        // 1 2 3 4 5 6 7
        // 5
        // 3 4 4 1 3
        // 3 4 5 6 7
        // 3
        // 2 1 1
        // 1 2 2
        // 4
        // 2 1 4 1
        // 1 1 1 1
        // "), "\
        // 1 2 3
        // 2 4 5 1 6 3 7 8
        // 3 4 5 1 2
        // 1 2
        // 7 5 1 3 2 6 4
        // 5 3 2 4 1
        // 3 2 1
        // 3 4 1 2");
    }
}
