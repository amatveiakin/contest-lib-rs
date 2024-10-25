use std::{array, iter};

use crate::base_one::Base;
use crate::graph::{VertexId, Graph, StorageVertexId};
use crate::{io, ivec};
use crate::undirected_graph::UndirectedGraph;


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TreeConstructionError {
    EmptyGraph,
    HasCycle,
    NotConnected,
}

#[derive(Clone, Debug)]
pub struct Tree<VP, EP> {
    vertices: Vec<TreeVertex<VP, EP>>,
    root: StorageVertexId,
}

#[derive(Clone, Debug)]
pub struct TreeVertex<VP, EP> {
    pub payload: VP,
    pub parent: Option<(StorageVertexId, EP)>,
    pub children: Vec<StorageVertexId>,
}

impl<VP, EP> Tree<VP, EP> {
    pub fn new_with_root_p(root_payload: VP) -> (Self, VertexId) {
        let v = TreeVertex {
            payload: root_payload,
            parent: None,
            children: Vec::new(),
        };
        let root = 0 as StorageVertexId;
        let tree = Tree { vertices: vec![v], root };
        (tree, root as VertexId)
    }

    pub fn add_child_p(&mut self, parent: VertexId, vertex_payload: VP, edge_payload: EP) -> VertexId {
        let v = TreeVertex {
            payload: vertex_payload,
            parent: Some((parent as StorageVertexId, edge_payload)),
            children: Vec::new(),
        };
        let id = self.vertices.len();
        self.vertices.push(v);
        self.vertices[parent].children.push(id as StorageVertexId);
        id as VertexId
    }

    pub fn root(&self) -> VertexId { self.root as VertexId }

    pub fn children(&self, v: VertexId) -> impl ExactSizeIterator<Item = VertexId> + '_ {
        self.vertices[v].children.iter().map(|&u| u as VertexId)
    }
    pub fn child_edges(&self, v: VertexId) -> impl ExactSizeIterator<Item = (VertexId, &EP)> {
        self.vertices[v].children.iter().map(
            |&u| (u as VertexId, &self.vertices[u as usize].parent.as_ref().unwrap().1))
    }
    pub fn parent(&self, v: VertexId) -> Option<VertexId> {
        self.vertices[v].parent.as_ref().map(|(p, _)| *p as VertexId)
    }
    pub fn parent_edge(&self, v: VertexId) -> Option<(VertexId, &EP)> {
        self.vertices[v].parent.as_ref().map(|(p, payload)| (*p as VertexId, payload))
    }
    pub fn silblings(&self, v: VertexId) -> Box<dyn Iterator<Item = VertexId> + '_> {
        self.vertices[v].parent.as_ref().map_or(
            Box::new(iter::empty()),
            |&(p, _)| Box::new(
                self.vertices[p as usize].children.iter()
                    .map(|&u| u as VertexId)
                    .filter(move |&u| u != v)))
    }

    // Improvement potential: Rename to `compute_bottom_up` add `compute_top_down`.
    pub fn compute_bottom_up<R, F>(&self, f: F) -> Vec<R>
    where
        F: Fn(&[&R], VertexId) -> R,
    {
        let mut result = ivec![None; self.vertices.len()];
        self.compute_bottom_up_impl(&f, self.root as VertexId, &mut result);
        result.into_iter().map(|v| v.unwrap()).collect()
    }

    fn compute_bottom_up_impl<F, R>(&self, f: &F, v: VertexId, result: &mut Vec<Option<R>>)
    where
        F: Fn(&[&R], VertexId) -> R,
    {
        for u in self.children(v) {
            self.compute_bottom_up_impl(f, u, result);
        }
        // Improvement potential: Pass an iterator instead of collecting to a vector.
        let children_results = self.children(v)
            .map(|u| result[u].as_ref().unwrap())
            .collect::<Vec<_>>();
        assert!(result[v].is_none());
        result[v] = Some(f(&children_results, v));
    }

    pub fn compute_top_down<R, F>(&self, root_value: R, f: F) -> Vec<R>
    where
        F: Fn(&R, VertexId) -> R,
    {
        let mut result = ivec![None; self.vertices.len()];
        self.compute_top_down_impl(&f, self.root as VertexId, root_value, &mut result);
        result.into_iter().map(|v| v.unwrap()).collect()
    }

    fn compute_top_down_impl<F, R>(
        &self, f: &F, v: VertexId, value: R, result: &mut Vec<Option<R>>)
    where
        F: Fn(&R, VertexId) -> R,
    {
        for u in self.children(v) {
            let child_value = f(&value, u);
            self.compute_top_down_impl(f, u, child_value, result);
        }
        assert!(result[v].is_none());
        result[v] = Some(value);
    }
}

impl<VP: Clone, EP: Clone> Tree<VP, EP> {
    // Not using `TryFrom` trait because it expects a value, not a reference.
    pub fn try_from(graph: &UndirectedGraph<VP, EP>) -> Result<Self, TreeConstructionError> {
        Self::try_from_with_root(graph, 0)
    }

    pub fn try_from_with_root(graph: &UndirectedGraph<VP, EP>, root: VertexId)
        -> Result<Self, TreeConstructionError>
    {
        if graph.num_vertices() == 0 {
            return Err(TreeConstructionError::EmptyGraph);
        }
        let root = root as StorageVertexId;
        let mut vertices = graph.vertex_ids().map(|v| TreeVertex {
            payload: graph.vertex(v).clone(),
            parent: None,
            children: Vec::new(),
        }).collect::<Vec<_>>();
        let mut found_vertices = 1;  // root is already ok
        let mut stack = vec![(root, None)];
        while let Some((v, p)) = stack.pop() {
            let v = v as VertexId;
            for (u, payload) in graph.edges_adj(v) {
                if Some(u) == p {
                    continue;
                }
                let was_visited = vertices[u].parent.is_some() || u == root as VertexId;
                if was_visited {
                    return Err(TreeConstructionError::HasCycle);
                }
                vertices[u].parent = Some((v as StorageVertexId, payload.clone()));
                vertices[v].children.push(u as StorageVertexId);
                found_vertices += 1;
                stack.push((u as StorageVertexId, Some(v)));
            }
        }
        if found_vertices < graph.num_vertices() {
            return Err(TreeConstructionError::NotConnected);
        }
        return Ok(Tree { vertices, root });
    }

    pub fn chroot(&self, new_root: VertexId) -> Tree<VP, EP> {
        // Cannot fail: we know this is a tree.
        Self::try_from_with_root(&self.to_graph(), new_root).unwrap()
    }

    pub fn to_graph(&self) -> UndirectedGraph<VP, EP> {
        let mut graph = UndirectedGraph::new();
        for v in self.vertex_ids() {
            graph.add_vertex_p(self.vertex(v).clone());
        }
        for (u, v, p) in self.edges() {
            graph.add_edge_p(u, v, p.clone());
        }
        graph
    }
}

impl<VP, EP> Graph<VP, EP> for Tree<VP, EP> {
    const IS_DIRECTED: bool = false;

    fn num_vertices(&self) -> usize { self.vertices.len() }
    fn num_edges(&self) -> usize { self.num_vertices() - 1 }

    fn edges<'g>(&'g self) -> impl Iterator<Item = (VertexId, VertexId, &'g EP)> where EP: 'g {
        self.vertices.iter().enumerate().flat_map(move |(to, vertex)| {
            vertex.parent.iter().map(move |(from, payload)| (*from as VertexId, to, payload))
        })
    }

    fn vertex(&self, v: VertexId) -> &VP { &self.vertices[v].payload }
    fn vertex_mut(&mut self, v: VertexId) -> &mut VP { &mut self.vertices[v].payload }

    fn edge(&self, from: VertexId, to: VertexId) -> Option<&EP> {
        if self.parent(from) == Some(to) {
            Some(&self.vertices[from].parent.as_ref().unwrap().1)
        } else if self.parent(to) == Some(from) {
            Some(&self.vertices[to].parent.as_ref().unwrap().1)
        } else {
            None
        }
    }
    fn edge_mut(&mut self, from: VertexId, to: VertexId) -> Option<&mut EP> {
        if self.parent(from) == Some(to) {
            Some(&mut self.vertices[from].parent.as_mut().unwrap().1)
        } else if self.parent(to) == Some(from) {
            Some(&mut self.vertices[to].parent.as_mut().unwrap().1)
        } else {
            None
        }
    }

    fn degree(&self, v: VertexId) -> usize {
        let vertex = &self.vertices[v];
        vertex.children.len() + vertex.parent.iter().count()
    }
    fn out_degree(&self, v: VertexId) -> usize { self.degree(v) }
    fn in_degree(&self, v: VertexId) -> usize { self.degree(v) }

    fn edges_adj<'g>(&'g self, v: VertexId) -> impl Iterator<Item = (VertexId, &'g EP)> where EP: 'g {
        self.child_edges(v).chain(self.parent_edge(v).into_iter())
    }
    fn edges_in<'g>(&'g self, to: VertexId) -> impl Iterator<Item = (VertexId, &'g EP)> where EP: 'g {
        self.edges_adj(to)
    }
    fn edges_out<'g>(&'g self, from: VertexId) -> impl Iterator<Item = (VertexId, &'g EP)> where EP: 'g {
        self.edges_adj(from)
    }
}

impl<EP: Clone> Tree<(), EP> {
    // Reads edges as 1-based vertex pairs.
    pub fn from_read_edges_p<R: std::io::BufRead>(
        num_vertices: usize, base: Base, read: &mut io::Reader<R>,
        read_payload: impl Fn(&mut io::Reader<R>) -> EP
    ) -> Self {
        let graph = UndirectedGraph::from_read_edges_p(
            num_vertices, num_vertices - 1, base, read, read_payload);
        Tree::try_from(&graph).unwrap()
    }
}

impl Tree<(), ()> {
    pub fn new_with_root() -> (Self, VertexId) {
        Self::new_with_root_p(())
    }

    pub fn add_child(&mut self, parent: VertexId) -> VertexId {
        self.add_child_p(parent, (), ())
    }

    pub fn add_children<const N: usize>(&mut self, parent: VertexId) -> [VertexId; N] {
        array::from_fn(|_| self.add_child(parent))
    }

    // Reads edges as 1-based vertex pairs.
    pub fn from_read_edges<R: std::io::BufRead>(
        num_vertices: usize, base: Base, read: &mut io::Reader<R>
    ) -> Self {
        Self::from_read_edges_p(num_vertices, base, read, |_| ())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use pretty_assertions::assert_eq;

    #[test]
    fn make_tree_ok() {
        let mut g = UndirectedGraph::new();
        let [v1, v2, v3, v4, v5, v6] = g.add_vertex_array();
        g.add_edge(v2, v3);
        g.add_edge(v1, v3);
        g.add_edge(v3, v4);
        g.add_edge(v6, v3);
        g.add_edge(v4, v5);
        let tree = Tree::try_from(&g).unwrap();
        assert_eq!(tree.root(), v1);
        assert_eq!(tree.parent(v1), None);
        assert_eq!(tree.children(v1).collect_vec(), vec![v3]);
        assert_eq!(tree.silblings(v1).collect_vec(), vec![]);
        assert_eq!(tree.parent(v3), Some(v1));
        assert_eq!(tree.children(v3).len(), 3);
        assert_eq!(tree.degree(v3), 4);
        assert_eq!(tree.edges_adj(v3).map(|(w, _)| w).sorted().collect_vec(), [v1, v2, v4, v6]);
        assert_eq!(tree.silblings(v4).sorted().collect_vec(), vec![v2, v6]);
    }

    #[test]
    fn make_tree_cycles() {
        let mut g = UndirectedGraph::new();
        let [v1, v2, v3] = g.add_vertex_array();
        g.add_edge(v1, v2);
        g.add_edge(v1, v3);
        g.add_edge(v2, v3);
        assert_eq!(Tree::try_from(&g).err(), Some(TreeConstructionError::HasCycle));
    }

    #[test]
    fn make_tree_unconnected() {
        let mut g = UndirectedGraph::new();
        let [v1, v2, v3, v4] = g.add_vertex_array();
        g.add_edge(v1, v3);
        g.add_edge(v2, v4);
        assert_eq!(Tree::try_from(&g).err(), Some(TreeConstructionError::NotConnected));
    }

    #[test]
    fn make_tree_with_payload() {
        let mut g = UndirectedGraph::new();
        let v1 = g.add_vertex_p("first");
        let v2 = g.add_vertex_p("second");
        g.add_edge_p(v1, v2, "first-second");
        let tree = Tree::try_from(&g).unwrap();
        assert_eq!(tree.vertex(v1), &"first");
        assert_eq!(tree.vertex(v2), &"second");
        assert_eq!(tree.parent_edge(v2).unwrap().1, &"first-second");
    }

    #[test]
    fn recursive_computation() {
        let mut graph = UndirectedGraph::new();
        let [a, b, c, d, e, f, g, h] = graph.add_vertex_array();
        graph.add_edge(a, b);
        graph.add_edge(a, c);
        graph.add_edge(a, d);
        graph.add_edge(c, e);
        graph.add_edge(c, f);
        graph.add_edge(d, g);
        graph.add_edge(g, h);
        let tree = Tree::try_from(&graph).unwrap();

        let subtree_sizes = tree.compute_bottom_up(|ch_sizes, _| {
            1 + ch_sizes.iter().copied().sum::<i64>()
        });
        assert_eq!(subtree_sizes[a], 8);
        assert_eq!(subtree_sizes[b], 1);
        assert_eq!(subtree_sizes[c], 3);
        assert_eq!(subtree_sizes[d], 3);
        assert_eq!(subtree_sizes[e], 1);
        assert_eq!(subtree_sizes[f], 1);
        assert_eq!(subtree_sizes[g], 2);
        assert_eq!(subtree_sizes[h], 1);

        let subtrees = tree.compute_bottom_up(|ch_subtrees, v| {
            let mut ret = vec![v];
            for ch in ch_subtrees {
                ret.extend(*ch);
            }
            ret.sort();
            ret
        });
        assert_eq!(subtrees[a], vec![a, b, c, d, e, f, g, h]);
        assert_eq!(subtrees[b], vec![b]);
        assert_eq!(subtrees[c], vec![c, e, f]);
        assert_eq!(subtrees[d], vec![d, g, h]);
        assert_eq!(subtrees[e], vec![e]);
        assert_eq!(subtrees[f], vec![f]);
        assert_eq!(subtrees[g], vec![g, h]);
        assert_eq!(subtrees[h], vec![h]);
    }
}
