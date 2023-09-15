use std::mem;


pub trait IterutilsDedup
where
    Self: Sized + Iterator,
{
    type GroupIdentical: Iterator<Item = (Self::Item, usize)> where Self::Item: PartialEq;

    // Similar to `Itertools::group_by`, but:
    //   - returns the first element and count;
    //   - returns an iterator; does not need to be stored in a temporary variable.
    fn group_identical(self) -> Self::GroupIdentical where Self::Item: PartialEq;
}


pub struct GroupIdentical<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    iter: I,
    last: Option<I::Item>,
    count: usize,
}

impl<I> Iterator for GroupIdentical<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    type Item = (I::Item, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            if let Some(ref last) = self.last {
                if *last == item {
                    self.count += 1;
                } else {
                    let prev = mem::replace(&mut self.last, Some(item)).unwrap();
                    let ret = Some((prev, self.count));
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
            let ret = Some((prev, self.count));
            self.count = 0;
            return ret;
        }
        None
    }
}

impl<I: Iterator> IterutilsDedup for I {
    type GroupIdentical = GroupIdentical<Self> where Self::Item: PartialEq;

    fn group_identical(self) -> Self::GroupIdentical where Self::Item: PartialEq {
        GroupIdentical {
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
    fn group_identical() {
        let v = [2, 2, 2, 1, 1, 3, 3, 3, 3, 1];
        let grouped: Vec<_> = v.iter().copied().group_identical().collect();
        assert!(grouped == vec![(2, 3), (1, 2), (3, 4), (1, 1)]);
    }

    #[test]
    fn group_identical_non_copiable() {
        let v = [String::from("foo"), String::from("foo"), String::from("bar")];
        let grouped: Vec<_> = v.iter().group_identical().collect();
        assert!(grouped == vec![(&String::from("foo"), 2), (&String::from("bar"), 1)]);
    }
}
