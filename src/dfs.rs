use std::ops;
use std::collections::{HashSet, HashMap, VecDeque};

use crate::graph::{Graph, VertexId};


// Generic DFS traversal.
//
// Q. Is this at all useful? For example, topological sort turned out to be easier to implement from
// scratch. Also in Rust specifically on_see/on_enter/on_exit thing in inconvenient, because these
// callbacks cannot access common state if at least one does so mutable. This could be mitigated by
// replacing the three callbacks with a single trait (but then using the DFS traversal will be even
// more cumbersome).
//
// TODO: Manual vertex stack (be careful about callback order!)
//
pub fn dfs<'g, VP, EP: 'g, R>(
    graph: &'g impl Graph<'g, VP, EP>, from: VertexId,
    mut on_see: impl FnMut(VertexId) -> ops::ControlFlow<R, ()>,
    mut on_enter: impl FnMut(VertexId) -> ops::ControlFlow<R, ()>,
    mut on_exit: impl FnMut(VertexId) -> ops::ControlFlow<R, ()>,
) -> Option<R> {
    let mut visited = HashSet::new();
    match dfs_impl(graph, from, &mut visited, &mut on_see, &mut on_enter, &mut on_exit) {
        ops::ControlFlow::Break(r) => Some(r),
        ops::ControlFlow::Continue(()) => None,
    }
}

fn dfs_impl<'g, VP, EP: 'g, R>(
    graph: &'g impl Graph<'g, VP, EP>,
    v: VertexId,
    visited: &mut HashSet<VertexId>,
    on_see: &mut impl FnMut(VertexId) -> ops::ControlFlow<R, ()>,
    on_enter: &mut impl FnMut(VertexId) -> ops::ControlFlow<R, ()>,
    on_exit: &mut impl FnMut(VertexId) -> ops::ControlFlow<R, ()>,
) -> ops::ControlFlow<R, ()> {
    on_enter(v)?;
    for e in graph.edges_from(v) {
        on_see(e.other)?;
        if visited.insert(v) {
            dfs_impl(graph, e.other, visited, on_see, on_enter, on_exit);
        }
    }
    on_exit(v)?;
    ops::ControlFlow::Continue(())
}
