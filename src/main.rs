// TODO: For the compiler: auto wrap each file in `mod`

#![allow(unused_imports)]

pub mod bfs;
pub mod directed_graph;
pub mod graph;
pub mod io;
pub mod partial_sums;
pub mod segment_tree;
pub mod testing;
pub mod u32_index;
pub mod undirected_graph;

use bfs::*;
use directed_graph::*;
use graph::*;
use io::*;
use partial_sums::*;
use segment_tree::*;
use u32_index::*;
use undirected_graph::*;


fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.i32();
    let v = read.vec_i32(n as usize);
    emitln!(write, n);
    emitln!(write, v);
}

fn main() {
    let mut read = Reader::new(std::io::stdin().lock());
    let mut write = std::io::stdout().lock();
    solve(&mut read, &mut write);
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use testing::run_solver;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "3\n1 2 3\n"), "3\n1 2 3\n");
    }
}
