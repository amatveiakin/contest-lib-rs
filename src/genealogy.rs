// Improvement potential. Find a way to compute auxiliary structures like DFS numbering and binary
// lifting in a way that:
//   - Each structure is computed once and reused by all algorithms which need it;
//   - No unnecessary structures are computed;
//
// This could be done in different ways (syntax is not final):
//
// (1)
//   let dfs_numbering = get_dfs_numbering(&tree);
//   let binary_lifting = get_binary_lifting(&tree);
//   let lca = LowestCommonAncestor::new(&dfs_numbering, &binary_lifting);
//   let c = lca.get(u, v);
//
// (2) (current)
//   let dfs_numbering = get_dfs_numbering(&tree);
//   let binary_lifting = get_binary_lifting(&tree);
//   let c = lowest_common_ancestor(u, v, &dfs_numbering, &binary_lifting);
//
// (3)
//   let (dfs_numbering, binary_lifting) =
//       tree_compute(&tree, (DfsNumbering::builder(), BinaryLifting::builder()));
//   let c = lowest_common_ancestor(u, v, &dfs_numbering, &binary_lifting);
//
// (4)
//   let tree = tree.compute_dfs_numbering();
//   let tree = tree.compute_binary_lifting();
//   let c = tree.lowest_common_ancestor(u, v);
//
// (5)
//   let tree = tree.compute::<DfsNumbering, BinaryLifting>();
//   let c = tree.lowest_common_ancestor(u, v);
//
// (6)
//   // Constructs simple wrapper in O(1) time.
//   let navigator = TreeNavigator::new(&tree);
//   // Lazily computes DFS numbering and binary lifting on first use.
//   let c = navigator.lowest_common_ancestor(u, v);
//
// Ideal solution properties:
//   - Ideally simple cases should be simple;
//   - Less DFS traversals is better.
//
// Options (1) and (2) are very easy to implement. Options (4) and (5) seem more convenient for the
// user, but require a way to figure out how to stuff all this information into the `Tree` type
// without making it too complex. Options (3) and (5) allow to do just a single DFS traversal.
// Option (1) could be made easier to use in the simple case by splitting into
//   `LowestCommonAncestor::new()` and
//   `LowestCommonAncestor::new_with_precomputed(Some(&dfs_numbering), None);`
// where you can pass the information that you have and the rest is calculated automatically
// Option (6) is easy to use and to implement, but is a bit too magical. I'm not sure I like the
// idea of hiding the set of auxiliary structures that are computed.

use std::ops;

use crate::graph::{VertexId, StorageVertexId, Graph};
use crate::tree::Tree;


pub struct DfsNumbering {
    numbering: Vec<(u32, u32)>,
}

pub struct BinaryLifting {
    lifting: Vec<Vec<StorageVertexId>>,
}

pub struct VertexDepths {
    depths: Vec<u32>,
}

impl DfsNumbering {
    pub fn new<VP, EP>(tree: &Tree<VP, EP>) -> Self {
        let mut timestamp = 0;
        let mut numbering = vec![(0, 0); tree.num_vertices()];
        compute_dfs_numbering(tree, tree.root(), &mut timestamp, &mut numbering);
        Self { numbering }
    }

    pub fn index_end(&self) -> usize {
        self.numbering.len()
    }

    pub fn range(&self, v: VertexId) -> ops::Range<u32> {
        let (l, r) = self.numbering[v];
        l..r
    }

    pub fn unique_point(&self, v: VertexId) -> u32 {
        let (l, _) = self.numbering[v];
        l
    }

    pub fn is_ancestor(&self, ancestor: VertexId, descendant: VertexId) -> bool {
        let (al, ar) = self.numbering[ancestor];
        let (dl, dr) = self.numbering[descendant];
        al <= dl && dr <= ar
    }

    // The total number of vertices in `v`'s subtree, including `v` itself.
    pub fn subtree_size(&self, v: VertexId) -> usize {
        let (l, r) = self.numbering[v];
        (r - l) as usize
    }
}

impl ops::Index<VertexId> for DfsNumbering {
    type Output = (u32, u32);
    fn index(&self, v: VertexId) -> &Self::Output {
        &self.numbering[v]
    }
}

impl BinaryLifting {
    pub fn new<VP, EP>(tree: &Tree<VP, EP>) -> Self {
        let mut lifting = vec![vec![]; tree.num_vertices()];
        compute_binary_lifting(tree, tree.root(), &mut lifting);
        Self { lifting }
    }
}

impl ops::Index<VertexId> for BinaryLifting {
    type Output = Vec<StorageVertexId>;
    fn index(&self, v: VertexId) -> &Self::Output {
        &self.lifting[v]
    }
}

impl VertexDepths {
    pub fn new<VP, EP>(tree: &Tree<VP, EP>) -> Self {
        let mut depths = vec![0; tree.num_vertices()];
        compute_vertex_depths(tree, tree.root(), 0, &mut depths);
        Self { depths }
    }
}

impl ops::Index<VertexId> for VertexDepths {
    type Output = u32;
    fn index(&self, v: VertexId) -> &Self::Output {
        &self.depths[v]
    }
}

pub fn lowest_common_ancestor(
    dfs_numbering: &DfsNumbering, binary_lifting: &BinaryLifting, u: VertexId, v: VertexId
) -> VertexId {
    if dfs_numbering.is_ancestor(u, v) {
        u
    } else if dfs_numbering.is_ancestor(v, u) {
        v
    } else {
        let mut u = u;
        'outer: loop {
            for &p in binary_lifting[u].iter().rev() {
                let p = p as VertexId;
                if !dfs_numbering.is_ancestor(p, v) {
                    u = p;
                    continue 'outer;
                }
            }
            return binary_lifting[u][0] as VertexId;
        }
    }
}

fn compute_dfs_numbering<VP, EP>(
    tree: &Tree<VP, EP>, v: VertexId, timestamp: &mut u32, numbering: &mut Vec<(u32, u32)>
) {
    let t0 = *timestamp;
    *timestamp += 1;
    for ch in tree.children(v) {
        compute_dfs_numbering(tree, ch, timestamp, numbering);
    }
    numbering[v] = (t0, *timestamp);
}

fn compute_binary_lifting<VP, EP>(
    tree: &Tree<VP, EP>, v: VertexId, lifting: &mut Vec<Vec<StorageVertexId>>
) {
    if let Some(p) = tree.parent(v) {
        let mut p = p as StorageVertexId;
        lifting[v].push(p);
        for i in 1.. {
            let Some(&pp) = lifting[lifting[v][i - 1] as usize].get(i - 1) else {
                break;
            };
            p = pp;
            lifting[v].push(p);
        }
    };
    for ch in tree.children(v) {
        compute_binary_lifting(tree, ch, lifting);
    }
}

fn compute_vertex_depths<VP, EP>(tree: &Tree<VP, EP>, v: VertexId, d: u32, depths: &mut Vec<u32>) {
    depths[v] = d;
    for ch in tree.children(v) {
        compute_vertex_depths(tree, ch, d + 1, depths)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let (mut tree, a) = Tree::new_with_root();
        let [b, c, d] = tree.add_children(a);
        let [e] = tree.add_children(b);
        let [f, g] = tree.add_children(d);
        let [h, k] = tree.add_children(e);
        let [l] = tree.add_children(h);
        let [m, n, o] = tree.add_children(l);

        let dfs_numbering = DfsNumbering::new(&tree);
        let binary_lifting = BinaryLifting::new(&tree);

        assert!(dfs_numbering.is_ancestor(a, a));
        assert!(dfs_numbering.is_ancestor(a, b));
        assert!(dfs_numbering.is_ancestor(a, o));
        assert!(dfs_numbering.is_ancestor(b, e));
        assert!(dfs_numbering.is_ancestor(b, k));
        assert!(dfs_numbering.is_ancestor(b, n));
        assert!(dfs_numbering.is_ancestor(d, f));
        assert!(!dfs_numbering.is_ancestor(c, a));
        assert!(!dfs_numbering.is_ancestor(b, f));
        assert!(!dfs_numbering.is_ancestor(b, c));
        assert!(!dfs_numbering.is_ancestor(c, b));

        assert_eq!(dfs_numbering.subtree_size(a), 13);
        assert_eq!(dfs_numbering.subtree_size(b), 8);
        assert_eq!(dfs_numbering.subtree_size(c), 1);
        assert_eq!(dfs_numbering.subtree_size(l), 4);
        assert_eq!(dfs_numbering.subtree_size(n), 1);

        assert!(lowest_common_ancestor(&dfs_numbering, &binary_lifting, a, a) == a);
        assert!(lowest_common_ancestor(&dfs_numbering, &binary_lifting, l, l) == l);
        assert!(lowest_common_ancestor(&dfs_numbering, &binary_lifting, b, d) == a);
        assert!(lowest_common_ancestor(&dfs_numbering, &binary_lifting, b, d) == a);
        assert!(lowest_common_ancestor(&dfs_numbering, &binary_lifting, b, m) == b);
        assert!(lowest_common_ancestor(&dfs_numbering, &binary_lifting, g, n) == a);
        assert!(lowest_common_ancestor(&dfs_numbering, &binary_lifting, f, g) == d);
    }
}
