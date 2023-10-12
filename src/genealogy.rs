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
// (2)
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
// where you can pass the information that you have and the rest is calculated automatically.

use crate::graph::{VertexId, StorageVertexId, Graph};
use crate::tree::Tree;


pub struct Genealogy {
    dfs_numbering: Vec<(u32, u32)>,
    binary_lifting: Vec<Vec<StorageVertexId>>,
}

impl Genealogy {
    pub fn new<VP, EP>(tree: &Tree<VP, EP>) -> Self {
        let mut dfs_timestamp = 0;
        let mut dfs_numbering = vec![(0, 0); tree.num_vertices()];
        let mut binary_lifting = vec![vec![]; tree.num_vertices()];
        dfs(tree.root(), tree, &mut dfs_timestamp, &mut dfs_numbering, &mut binary_lifting);
        Self {
            dfs_numbering,
            binary_lifting,
        }
    }

    pub fn dfs_numbering(&self) -> &[(u32, u32)] { &self.dfs_numbering }
    pub fn binary_lifting(&self) -> &[Vec<StorageVertexId>] { &self.binary_lifting }

    pub fn is_parent(&self, parent: VertexId, child: VertexId) -> bool {
        let (p0, p1) = self.dfs_numbering[parent];
        let (c0, c1) = self.dfs_numbering[child];
        p0 <= c0 && c1 <= p1
    }

    pub fn lowest_common_ancestor(&self, u: VertexId, v: VertexId) -> VertexId {
        if self.is_parent(u, v) {
            u
        } else if self.is_parent(v, u) {
            v
        } else {
            let mut u = u;
            'outer: loop {
                for &p in self.binary_lifting[u].iter().rev() {
                    let p = p as VertexId;
                    if !self.is_parent(p, v) {
                        u = p;
                        continue 'outer;
                    }
                }
                return self.binary_lifting[u][0] as VertexId;
            }
        }
    }
}

fn dfs<VP, EP>(
    v: VertexId, tree: &Tree<VP, EP>, dfs_timestamp: &mut u32,
    dfs_numbering: &mut Vec<(u32, u32)>, binary_lifting: &mut Vec<Vec<StorageVertexId>>
) {
    let t0 = *dfs_timestamp;
    *dfs_timestamp += 1;
    if let Some(p) = tree.parent(v) {
        let mut p = p as StorageVertexId;
        binary_lifting[v].push(p);
        for i in 1.. {
            let Some(&pp) = binary_lifting[binary_lifting[v][i - 1] as usize].get(i - 1) else {
                break;
            };
            p = pp;
            binary_lifting[v].push(p);
        }
    };
    for ch in tree.children(v) {
        dfs(ch, tree, dfs_timestamp, dfs_numbering, binary_lifting);
    }
    dfs_numbering[v] = (t0, *dfs_timestamp);
    *dfs_timestamp += 1;
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

        let genealogy = Genealogy::new(&tree);
        assert!(genealogy.is_parent(a, a));
        assert!(genealogy.is_parent(a, b));
        assert!(genealogy.is_parent(a, o));
        assert!(genealogy.is_parent(b, e));
        assert!(genealogy.is_parent(b, k));
        assert!(genealogy.is_parent(b, n));
        assert!(genealogy.is_parent(d, f));
        assert!(!genealogy.is_parent(c, a));
        assert!(!genealogy.is_parent(b, f));
        assert!(!genealogy.is_parent(b, c));
        assert!(!genealogy.is_parent(c, b));

        assert!(genealogy.lowest_common_ancestor(a, a) == a);
        assert!(genealogy.lowest_common_ancestor(l, l) == l);
        assert!(genealogy.lowest_common_ancestor(b, d) == a);
        assert!(genealogy.lowest_common_ancestor(b, d) == a);
        assert!(genealogy.lowest_common_ancestor(b, m) == b);
        assert!(genealogy.lowest_common_ancestor(g, n) == a);
        assert!(genealogy.lowest_common_ancestor(f, g) == d);
    }
}
