use std::mem;


pub trait IterutilsDedup
where
    Self: Sized + Iterator,
{
    type DedupWithCount: Iterator<Item = (usize, Self::Item)> where Self::Item: PartialEq;

    // Similar to `Itertools::group_by`, but:
    //   - returns the first element and count;
    //   - returns an iterator; does not need to be stored in a temporary variable.
    fn dedup_with_count(self) -> Self::DedupWithCount where Self::Item: PartialEq;
}


pub struct DedupWithCount<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    iter: I,
    last: Option<I::Item>,
    count: usize,
}

impl<I> Iterator for DedupWithCount<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    type Item = (usize, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            if let Some(ref last) = self.last {
                if *last == item {
                    self.count += 1;
                } else {
                    let prev = mem::replace(&mut self.last, Some(item)).unwrap();
                    let ret = Some((self.count, prev));
                    self.count = 1;
                    return ret;
                }
            } else {
                self.last = Some(item);
                self.count = 1;
            }
        }
        if self.last.is_some() {
            let prev = mem::replace(&mut self.last, None).unwrap();
            let ret = Some((self.count, prev));
            self.count = 0;
            return ret;
        }
        None
    }
}

impl<I: Iterator> IterutilsDedup for I {
    type DedupWithCount = DedupWithCount<Self> where Self::Item: PartialEq;

    fn dedup_with_count(self) -> Self::DedupWithCount where Self::Item: PartialEq {
        DedupWithCount {
            iter: self,
            last: None,
            count: 0,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dedup_with_count() {
        let v = [2, 2, 2, 1, 1, 3, 3, 3, 3, 1];
        let grouped: Vec<_> = v.iter().copied().dedup_with_count().collect();
        assert!(grouped == vec![(3, 2), (2, 1), (4, 3), (1, 1)]);
    }

    #[test]
    fn dedup_with_count_non_copiable() {
        let v = [String::from("foo"), String::from("foo"), String::from("bar")];
        let grouped: Vec<_> = v.iter().dedup_with_count().collect();
        assert!(grouped == vec![(2, &String::from("foo")), (1, &String::from("bar"))]);
    }
}
