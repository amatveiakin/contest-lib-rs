use std::ops;


const NODE_MISSING: usize = usize::MAX;

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    prev: usize,
    next: usize,
}

// A linked list that starts with several elements and supports removals and random access via
// original indices, but not insertions.
// TODO: Consider using parallel arrays for prev and next. This seems to have reduces
// LinkedListOnVec RAM usage.
#[derive(Debug, Clone)]
pub struct LinkedVector<T> {
    nodes: Vec<Option<Node<T>>>,
    first: usize,
    last: usize,
}

impl<T> LinkedVector<T> {
    pub fn original_len(&self) -> usize {
        self.nodes.len()
    }

    pub fn contains(&self, index: usize) -> bool {
        self.nodes[index].is_some()
    }

    pub fn first(&self) -> Option<usize> { to_option(self.first) }
    pub fn last(&self) -> Option<usize> { to_option(self.last) }

    pub fn next(&self, index: usize) -> Option<usize> {
        to_option(self.nodes[index].as_ref().unwrap().next)
    }
    pub fn prev(&self, index: usize) -> Option<usize> {
        to_option(self.nodes[index].as_ref().unwrap().prev)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.nodes[index].as_ref().map(|node| &node.value)
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        let node = self.nodes[index].take()?;
        if node.prev != NODE_MISSING {
            self.nodes[node.prev].as_mut().unwrap().next = node.next;
        } else {
            self.first = node.next;
        }
        if node.next != NODE_MISSING {
            self.nodes[node.next].as_mut().unwrap().prev = node.prev;
        } else {
            self.last = node.prev;
        }
        Some(node.value)
    }
}

impl<T> ops::Index<usize> for LinkedVector<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl LinkedVector<()> {
    pub fn new(len: usize) -> Self {
        (0..len).map(|_| ()).collect()
    }
}

impl<T> FromIterator<T> for LinkedVector<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut nodes: Vec<_> = iter
            .into_iter()
            .enumerate()
            .map(|(i, x)| Some(Node {
                value: x,
                prev: if i == 0 { NODE_MISSING } else { i - 1 },
                next: i + 1,
            }))
            .collect();
        let n = nodes.len();
        if n == 0 {
            Self { nodes, first: NODE_MISSING, last: NODE_MISSING }
        } else {
            nodes[n - 1].as_mut().unwrap().next = NODE_MISSING;
            Self { nodes, first: 0, last: n - 1 }
        }
    }
}

impl<T> From<Vec<T>> for LinkedVector<T> {
    fn from(vec: Vec<T>) -> Self {
        vec.into_iter().collect()
    }
}

fn to_option(index: usize) -> Option<usize> {
    (index != NODE_MISSING).then_some(index)
}
