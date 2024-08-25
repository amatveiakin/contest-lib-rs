use contest_lib_rs::graph::Graph;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::undirected_graph::UndirectedGraph;

#[derive(Clone, Copy, Debug)]
struct Circle {
    x: i64,
    y: i64,
    r: i64,
}

fn touches(a: Circle, b: Circle) -> bool {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let d2 = dx * dx + dy * dy;
    d2 <= (a.r + b.r).pow(2)
}

fn dfs(g: &UndirectedGraph<(), ()>, c: &mut [i32], v: usize, p: usize, color: i32) -> bool {
    if c[v] == -color {
        return false;
    }
    if c[v] == color {
        return true;
    }
    assert_eq!(c[v], 0);
    c[v] = color;
    for (u, ()) in g.edges_out(v) {
        if u == p {
            continue;
        }
        if !dfs(g, c, u, v, -color) {
            return false;
        }
    }
    true
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let circ = (0..n).map(|_| {
        let x = read.i64();
        let y = read.i64();
        let r = read.i64();
        Circle { x, y, r }
    }).collect_vec();

    let mut g = UndirectedGraph::new();
    g.add_vertices(n);

    for i in 0..n {
        for j in 0..i {
            if touches(circ[i], circ[j]) {
                g.add_edge(i, j);
            }
        }
    }

    for i in 0..n {
        let mut c = vec![0; n];
        if dfs(&g, &mut c, i, n, -1) {
            if c.iter().copied().sum::<i32>() < 0 {
                writeln!(write, "YES").ok();
                return;
            }
        }
    }
    writeln!(write, "NO").ok();
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
        assert_trimmed_eq!(&run_solver(solve, "\
        5
        0 2 1
        0 0 1
        4 -3 4
        11 0 3
        11 5 2"), "YES");
        assert_trimmed_eq!(&run_solver(solve, "\
        4
        2 2 2
        7 2 3
        7 7 2
        2 7 3"), "NO");
    }
}
