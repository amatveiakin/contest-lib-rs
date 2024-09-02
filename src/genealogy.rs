// Implements the following low-level helper classes:
//
//   * `DfsNumbering`: O(V) space and build time
//     For each vertex, stores enter and exit times of a DFS traversal.
//
//   * `BinaryLifting`: O(V log V) space and build time
//     For each vertex, stores the list of ancestors at distances 2^0, 2^1, 2^2, ...
//
//   * `VertexDepths`: O(V) space and build time
//     For each vertex, stores the distance to the root.
//
// and the following high-level helper functions:
//
//   * `lowest_common_ancestor(DfsNumbering, BinaryLifting, u, v)`: O(log V)
//     Lowest common ancestor of two vertices.
//
//   * `tree_path_via_depths(Tree, VertexDepths, from, to)`: O(path_length)
//   * `tree_path_via_dfsn(Tree, DfsNumbering, from, to)`: O(path_length)
//     Path in a tree. Includes the start and end vertices.
//
//   * `tree_distance(DfsNumbering, BinaryLifting, VertexDepths, from, to)`: O(log V)
//     Equals to path length minus one.

// Improvement potential. Consider replacing with this syntax:
//   let genealogy = compute_genealogy::<DfsNumbering, BinaryLifting>(&tree);
//   let p = genealogy.lowest_common_ancestor(u, v);
// Benefits:
//   - Compact syntax (less structures, less arguments to pass around)
//   - Potentially possible to compute all structures with a single DFS.
// The question is if Rust generic system is rich enough to support this.

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

pub fn tree_path_via_depths<VP, EP>(
    tree: &Tree<VP, EP>, vertex_depths: &VertexDepths, mut from: VertexId, mut to: VertexId,
) -> Vec<VertexId> {
    let mut path_from = vec![];
    let mut path_to = vec![];
    while vertex_depths[from] > vertex_depths[to] {
        path_from.push(from);
        from = tree.parent(from).unwrap();
    }
    while vertex_depths[to] > vertex_depths[from] {
        path_to.push(to);
        to = tree.parent(to).unwrap();
    }
    while from != to {
        path_from.push(from);
        path_to.push(to);
        from = tree.parent(from).unwrap();
        to = tree.parent(to).unwrap();
    }
    path_from.push(from);
    path_to.reverse();
    path_from.extend_from_slice(&path_to);
    path_from
}

pub fn tree_path_via_dfsn<VP, EP>(
    tree: &Tree<VP, EP>, dfs_numbering: &DfsNumbering, mut from: VertexId, mut to: VertexId,
) -> Vec<VertexId> {
    let mut path_from = vec![];
    let mut path_to = vec![];
    while !dfs_numbering.is_ancestor(from, to) {
        path_from.push(from);
        from = tree.parent(from).unwrap();
    }
    let ancestor = from;
    path_from.push(ancestor);
    while to != ancestor {
        path_to.push(to);
        to = tree.parent(to).unwrap();
    }
    path_to.reverse();
    path_from.extend_from_slice(&path_to);
    path_from
}

pub fn tree_distance(
    dfs_numbering: &DfsNumbering, binary_lifting: &BinaryLifting, vertex_depths: &VertexDepths,
    from: VertexId, to: VertexId,
) -> u32 {
    let ancestor = lowest_common_ancestor(dfs_numbering, binary_lifting, from, to);
    let d_from = vertex_depths[from] - vertex_depths[ancestor];
    let d_to = vertex_depths[to] - vertex_depths[ancestor];
    d_from + d_to
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
    use crate::bfs::bfs_path;

    use super::*;

    fn to_vertex_ids(storage_vertex_ids: &[StorageVertexId]) -> Vec<VertexId> {
        storage_vertex_ids.iter().map(|&v| v as VertexId).collect()
    }

    //            _____ a _____
    //           /      |      \
    //          b       c       d
    //          |             /   \
    //          e            f     g
    //        /   \
    //       h     k
    //       |
    //       l
    //     / | \
    //    m  n  o
    //
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
        let vertex_depths = VertexDepths::new(&tree);

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

        assert_eq!(to_vertex_ids(&binary_lifting[a]), []);
        assert_eq!(to_vertex_ids(&binary_lifting[b]), [a]);
        assert_eq!(to_vertex_ids(&binary_lifting[m]), [l, h, b]);

        assert_eq!(vertex_depths[a], 0);
        assert_eq!(vertex_depths[c], 1);
        assert_eq!(vertex_depths[k], 3);

        assert!(lowest_common_ancestor(&dfs_numbering, &binary_lifting, a, a) == a);
        assert!(lowest_common_ancestor(&dfs_numbering, &binary_lifting, l, l) == l);
        assert!(lowest_common_ancestor(&dfs_numbering, &binary_lifting, b, d) == a);
        assert!(lowest_common_ancestor(&dfs_numbering, &binary_lifting, b, d) == a);
        assert!(lowest_common_ancestor(&dfs_numbering, &binary_lifting, b, m) == b);
        assert!(lowest_common_ancestor(&dfs_numbering, &binary_lifting, g, n) == a);
        assert!(lowest_common_ancestor(&dfs_numbering, &binary_lifting, f, g) == d);

        for i in tree.vertex_ids() {
            for j in tree.vertex_ids() {
                let bfs_path = bfs_path(&tree, i, j).unwrap();
                let path1 = tree_path_via_depths(&tree, &vertex_depths, i, j);
                let path2 = tree_path_via_dfsn(&tree, &dfs_numbering, i, j);
                let dist = tree_distance(&dfs_numbering, &binary_lifting, &vertex_depths, i, j);
                assert_eq!(path1, bfs_path);
                assert_eq!(path2, bfs_path);
                assert_eq!(dist, bfs_path.len() as u32 - 1);
            }
        }
    }
}
