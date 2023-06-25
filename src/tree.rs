use crate::graph::{VertexId, Graph, HalfEdge};
use crate::io;
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
    root: VertexId,
}

#[derive(Clone, Debug)]
pub struct TreeVertex<VP, EP> {
    pub payload: VP,
    pub parent: Option<(VertexId, EP)>,
    pub children: Vec<VertexId>,
}

impl<VP, EP> Tree<VP, EP> {
    pub fn root(&self) -> VertexId { self.root }

    pub fn children(&self, v: VertexId) -> &[VertexId] { &self.vertices[v].children }
    pub fn child_edges(&self, v: VertexId) -> impl ExactSizeIterator<Item = HalfEdge<'_, EP>> {
        self.vertices[v].children.iter().map(|&u| HalfEdge {
            other: u,
            payload: &self.vertices[u].parent.as_ref().unwrap().1
        })
    }
    pub fn parent(&self, v: VertexId) -> Option<VertexId> {
        self.vertices[v].parent.as_ref().map(|(p, _)| *p)
    }
    pub fn parent_edge(&self, v: VertexId) -> Option<HalfEdge<'_, EP>> {
        self.vertices[v].parent.as_ref().map(|(p, payload)| HalfEdge {
            other: *p,
            payload,
        })
    }

    pub fn edges_adj(&self, v: VertexId) -> impl Iterator<Item = HalfEdge<'_, EP>> {
        self.child_edges(v).chain(self.parent_edge(v).into_iter())
    }
}

impl<VP: Clone, EP: Clone> Tree<VP, EP> {
    // Note using `TryFrom` trait because it expects a value, not a reference.
    pub fn try_from(graph: &UndirectedGraph<VP, EP>) -> Result<Self, TreeConstructionError> {
        if graph.num_vertices() == 0 {
            return Err(TreeConstructionError::EmptyGraph);
        }
        let root = VertexId::from_0_based(0);
        let mut vertices = graph.vertex_ids().map(|v| TreeVertex {
            payload: graph.vertex(v).clone(),
            parent: None,
            children: Vec::new(),
        }).collect::<Vec<_>>();
        let mut found_vertices = 1;  // root is already ok
        let mut stack = vec![(root, None)];
        while let Some((v, p)) = stack.pop() {
            for HalfEdge{ other: u, payload } in graph.edges_adj(v) {
                if Some(u) == p {
                    continue;
                }
                let was_visited = vertices[u].parent.is_some() || u == root;
                if was_visited {
                    return Err(TreeConstructionError::HasCycle);
                }
                vertices[u].parent = Some((v, payload.clone()));
                vertices[v].children.push(u);
                found_vertices += 1;
                stack.push((u, Some(v)));
            }
        }
        if found_vertices < graph.num_vertices() {
            return Err(TreeConstructionError::NotConnected);
        }
        return Ok(Tree { vertices, root });
    }
}

impl<'g, VP, EP: 'g> Graph<'g, VP, EP> for Tree<VP, EP> {
    type VertexIter = Box<dyn Iterator<Item = VertexId>>;
    type HalfEdgeIter = Box<dyn Iterator<Item = HalfEdge<'g, EP>> + 'g>;

    fn num_vertices(&self) -> usize { self.vertices.len() }

    fn vertex_ids(&self) -> Self::VertexIter {
        Box::new((0..self.vertices.len()).map(|i| VertexId::from_0_based(i.try_into().unwrap())))
    }

    fn vertex(&'g self, v: VertexId) -> &'g VP { &self.vertices[v].payload }
    fn vertex_mut(&'g mut self, v: VertexId) -> &'g mut VP { &mut self.vertices[v].payload }

    fn edge(&'g self, from: VertexId, to: VertexId) -> Option<&'g EP> {
        if self.parent(from) == Some(to) {
            Some(&self.vertices[from].parent.as_ref().unwrap().1)
        } else if self.parent(to) == Some(from) {
            Some(&self.vertices[to].parent.as_ref().unwrap().1)
        } else {
            None
        }
    }
    fn edge_mut(&'g mut self, from: VertexId, to: VertexId) -> Option<&'g mut EP> {
        if self.parent(from) == Some(to) {
            Some(&mut self.vertices[from].parent.as_mut().unwrap().1)
        } else if self.parent(to) == Some(from) {
            Some(&mut self.vertices[to].parent.as_mut().unwrap().1)
        } else {
            None
        }
    }

    fn degree(&'g self, v: VertexId) -> u32 {
        let vertex = &self.vertices[v];
        (vertex.children.len() + vertex.parent.iter().count()) as u32
    }
    fn out_degree(&'g self, v: VertexId) -> u32 { self.degree(v) }
    fn in_degree(&'g self, v: VertexId) -> u32 { self.degree(v) }

    fn edges_in(&'g self, to: VertexId) -> Self::HalfEdgeIter { Box::new(self.edges_adj(to)) }
    fn edges_out(&'g self, from: VertexId) -> Self::HalfEdgeIter { Box::new(self.edges_adj(from)) }
}

impl Tree<(), ()> {
    // Reads edges as 1-based vertex pairs.
    pub fn from_read_edges<R: std::io::BufRead>(num_vertices: usize, read: &mut io::Reader<R>)
        -> Result<Self, TreeConstructionError>
    {
        let graph = UndirectedGraph::from_read_edges(num_vertices, num_vertices - 1, read);
        Tree::try_from(&graph)
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
        assert_eq!(tree.children(v1), &[v3]);
        assert_eq!(tree.parent(v3), Some(v1));
        assert_eq!(tree.children(v3).len(), 3);
        assert_eq!(tree.degree(v3), 4);
        assert_eq!(tree.edges_adj(v3).map(|e| e.other).sorted().collect_vec(), [v1, v2, v4, v6]);
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
        assert_eq!(tree.parent_edge(v2).unwrap().payload, &"first-second");
    }
}
