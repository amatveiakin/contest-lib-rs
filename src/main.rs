// TODO: For the compiler: auto wrap each file in `mod`

#![allow(unused_imports)]

pub mod bfs;
pub mod dfs;
pub mod dijkstra;
pub mod directed_graph;
pub mod graph;
#[macro_use]
pub mod io;
pub mod num;
pub mod partial_sums;
pub mod segment_tree;
pub mod testing;
pub mod topological_sort;
pub mod u32_index;
pub mod undirected_graph;

use bfs::*;
use dfs::*;
use dijkstra::*;
use directed_graph::*;
use graph::*;
use io::*;
use num::*;
use partial_sums::*;
use segment_tree::*;
use topological_sort::*;
use u32_index::*;
use undirected_graph::*;


#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    // ...
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
        // assert_trimmed_eq!(&run_solver(solve, "..."), "...");
    }
}
