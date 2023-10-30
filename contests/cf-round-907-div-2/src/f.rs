use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::graph::{VertexId, Graph};
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::segment_tree::{SegmentTree, new_sum_tree};
use contest_lib_rs::tree::Tree;

fn dfs<F: Fn(&i64, &i64, i32) -> i64>(
    u: VertexId, t: &Tree::<(u32, Vec<(u32, i64)>), ()>, s: &mut SegmentTree<i64, F>, ans: &mut Vec<i64>
) {
    for &(iq, x) in &t.vertex(u).1 {
        s.update(iq, &x);
    }
    ans[u] = s.get(t.vertex(u).0..);
    for v in t.children(u) {
        dfs(v, t, s, ans);
    }
    for &(iq, x) in &t.vertex(u).1 {
        s.update(iq, &-x);
    }
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let q = read.u32();
    let (mut t, _) = Tree::<(u32, Vec<(u32, i64)>), ()>::new_with_root_p((0, vec![]));
    for iq in 1..=q {
        match read.u32() {
            1 => {
                let v = read.u32().from1b() as VertexId;
                t.add_child_p(v, (iq, vec![]), ());
            }
            2 => {
                let v = read.u32().from1b() as VertexId;
                let x = read.i64();
                t.vertex_mut(v).1.push((iq, x));
            }
            _ => unreachable!(),
        }
    }
    let mut ans = vec![0; t.num_vertices()];
    let mut s = new_sum_tree(&vec![0; q as usize + 1]);
    dfs(0, &t, &mut s, &mut ans);
    emitln!(write, ans);
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
        assert_trimmed_eq!(&run_solver(solve, "\
        3
        9
        2 1 3
        1 1
        2 2 1
        1 1
        2 3 2
        1 3
        2 1 4
        1 3
        2 3 2
        5
        2 1 1
        1 1
        2 1 -1
        1 1
        2 1 1
        5
        1 1
        1 1
        2 1 1
        2 1 3
        2 2 10
        "), "\
        7 5 8 6 2
        1 0 1
        4 14 4 ");
    }
}
