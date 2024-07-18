pub struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let size = vec![1; n];
        Self { parent, size }
    }

    pub fn len(&self) -> usize {
        self.parent.len()
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            return false;
        }
        if self.size[x] < self.size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.size[x] += self.size[y];
        self.parent[y] = x;
        true
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn sizes(ds: &mut DisjointSet) -> Vec<usize> {
        (0..ds.len()).map(|i| ds.size(i)).collect()
    }

    #[test]
    fn basic() {
        let mut ds = DisjointSet::new(7);
        assert_eq!(sizes(&mut ds), vec![1, 1, 1, 1, 1, 1, 1]);
        assert!(!ds.same(0, 1));
        assert!(!ds.same(1, 2));
        assert!(!ds.same(2, 3));
        assert!(!ds.same(3, 4));
        assert!(!ds.same(4, 0));

        assert!(ds.unite(0, 1));
        assert!(!ds.unite(0, 1));
        assert!(!ds.unite(1, 0));
        assert_eq!(sizes(&mut ds), vec![2, 2, 1, 1, 1, 1, 1]);

        assert!(ds.unite(2, 4));
        assert!(ds.unite(4, 6));
        assert_eq!(sizes(&mut ds), vec![2, 2, 3, 1, 3, 1, 3]);

        assert!(ds.same(0, 1));
        assert!(!ds.same(0, 2));
        assert!(!ds.same(0, 3));
        assert!(!ds.same(0, 4));
        assert!(!ds.same(0, 5));
        assert!(!ds.same(0, 6));
        assert!(ds.same(2, 6));

        assert!(ds.unite(4, 1));
        assert_eq!(sizes(&mut ds), vec![5, 5, 5, 1, 5, 1, 5]);
    }
}
