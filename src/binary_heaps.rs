use std::{cmp, ops};
use std::collections::BinaryHeap;


pub type MaxHeap<T> = BinaryHeap<T>;

#[derive(Clone, Default, Debug)]
pub struct MinHeap<T: Ord> {
    pub heap: BinaryHeap<cmp::Reverse<T>>,
}

pub struct MinHeapPeekMut<'a, T: 'a + Ord> {
    internal: std::collections::binary_heap::PeekMut<'a, cmp::Reverse<T>>,
}

impl<T: Ord> MinHeap<T> {
    pub fn append(&mut self, other: &mut Self) { self.heap.append(&mut other.heap); }
    // pub fn as_slice(&self) -> &[T]  // use `self.iter()` or `self.heap.slice()` plus manual `Reverse` unwrap instead
    pub fn capacity(&self) -> usize { self.heap.capacity() }
    pub fn clear(&mut self) { self.heap.clear(); }
    pub fn drain(&mut self) -> impl ExactSizeIterator<Item = T> + '_ { self.heap.drain().map(|x| x.0) }
    pub fn into_sorted_vec(self) -> Vec<T> { self.heap.into_sorted_vec().into_iter().map(|x| x.0).collect() }
    pub fn into_vec(self) -> Vec<T> { self.heap.into_vec().into_iter().map(|x| x.0).collect() }
    pub fn is_empty(&self) -> bool { self.heap.is_empty() }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &T> { self.heap.iter().map(|x| &x.0) }
    pub fn len(&self) -> usize { self.heap.len() }
    pub fn new() -> Self { Self { heap: BinaryHeap::new() } }
    pub fn peek(&self) -> Option<&T> { self.heap.peek().map(|x| &x.0) }
    pub fn peek_mut(&mut self) -> Option<MinHeapPeekMut<'_, T>> { self.heap.peek_mut().map(|internal| MinHeapPeekMut { internal }) }
    pub fn pop(&mut self) -> Option<T> { self.heap.pop().map(|x| x.0) }
    pub fn push(&mut self, x: T) { self.heap.push(cmp::Reverse(x)); }
    pub fn reserve(&mut self, additional: usize) { self.heap.reserve(additional); }
    pub fn reserve_exact(&mut self, additional: usize) { self.heap.reserve_exact(additional); }
    pub fn shrink_to(&mut self, min_capacity: usize) { self.heap.shrink_to(min_capacity); }
    pub fn shrink_to_fit(&mut self) { self.heap.shrink_to_fit(); }
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.heap.try_reserve(additional) }
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.heap.try_reserve_exact(additional) }
    pub fn with_capacity(capacity: usize) -> Self { Self { heap: BinaryHeap::with_capacity(capacity) } }
}

impl<T: Ord> ops::Deref for MinHeapPeekMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &T { &self.internal.deref().0 }
}
impl<T: Ord> ops::DerefMut for MinHeapPeekMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T { &mut self.internal.deref_mut().0 }
}
impl<'a, T: Ord> MinHeapPeekMut<'a, T> {
    pub fn pop(this: MinHeapPeekMut<'a, T>) -> T {
        std::collections::binary_heap::PeekMut::pop(this.internal).0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.push(2);
        heap.push(1);
        heap.push(3);
        assert_eq!(heap.peek(), Some(&1));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_min_heap_peek_mut() {
        let mut heap = MinHeap::new();
        heap.push(1);
        heap.push(2);
        heap.push(2);
        heap.push(2);
        heap.push(3);
        *heap.peek_mut().unwrap() += 2;
        *heap.peek_mut().unwrap() += 3;
        assert_eq!(MinHeapPeekMut::pop(heap.peek_mut().unwrap()), 2);
        assert_eq!(heap.into_sorted_vec(), vec![5, 3, 3, 2]);
    }
}
