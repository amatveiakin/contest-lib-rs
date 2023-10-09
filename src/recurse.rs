// `recurse` allows to write recursive closures.
//
// `f` may capture variables from its environment immutably. If mutable captures are needed, a
// `RefCell` can be used.
//
// See also:
//   - `memoize` for a version with caching.
//   - `Tree::compute_recursively` for a way to compute tree vertex properties that depend only on
//     children.

use crate::callable::Callable;


pub struct Recurser<T, U, F>
where
    F: Fn(T, &dyn Callable<T, U>) -> U,
{
    func: F,
    phantom: std::marker::PhantomData<(T, U)>,
}

impl<T, U, F> Callable<T, U> for Recurser<T, U, F>
where
    F: Fn(T, &dyn Callable<T, U>) -> U,
{
    fn call(&self, arg: T) -> U {
        (self.func)(arg, self)
    }
}

pub fn recurse<T, U, F>(func: F) -> Recurser<T, U, F>
where
    F: Fn(T, &dyn Callable<T, U>) -> U,
{
    Recurser {
        func,
        phantom: std::marker::PhantomData
    }
}


#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::graph::{VertexId, Graph};
    use crate::tree::Tree;
    use crate::undirected_graph::UndirectedGraph;

    use super::*;

    fn test_tree() -> Tree<(), ()> {
        let mut graph = UndirectedGraph::new();
        let [v1, v2, v3, v4, v5, v6] = graph.add_vertex_array();
        graph.add_edge(v1, v2);
        graph.add_edge(v1, v3);
        graph.add_edge(v2, v4);
        graph.add_edge(v3, v5);
        graph.add_edge(v5, v6);
        Tree::try_from(&graph).unwrap()
    }

    // Note. This is an illustration. `Tree::compute_recursively` would be a better way to do this.
    #[test]
    fn recursive_dfs_mutability_via_ref_cell() {
        let tree = test_tree();
        let root_distance = RefCell::new(vec![0; tree.num_vertices()]);
        recurse(|u: VertexId, f| {
            for v in tree.children(u) {
                // Note: read-borrow and write-borrow as separate statements! This would panic:
                //   root_distance.borrow_mut()[v] = root_distance.borrow()[u] + 1;  // PANIC
                let d = root_distance.borrow()[u] + 1;
                root_distance.borrow_mut()[v] = d;
                f.call(v);
            }
        }).call(0);
        let root_distance = root_distance.into_inner();
        assert_eq!(root_distance, vec![0, 1, 1, 2, 2, 3]);
    }

    // TODO: Make this work. If a regular recursive function can do this, why can't we?
    //
    // #[test]
    // fn recursive_dfs_mutability_via_arguments() {
    //     let tree = test_tree();
    //     let mut root_distance = vec![0; tree.num_vertices()];
    //     recurse(|(u, root_distance): (VertexId, &mut Vec<u32>), f| {
    //         for v in tree.children(u) {
    //             root_distance[v] = root_distance[u] + 1;
    //             f.call((v, root_distance));
    //         }
    //     }).call((0, &mut root_distance));
    //     assert_eq!(root_distance, vec![0, 1, 1, 2, 2, 3]);
    // }
}
