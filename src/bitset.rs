use std::{ops, fmt};


#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Bitset {
    words: Vec<u64>,  // trailing bits are always zero
    len: usize,       // length in bits
}

pub struct BitsetIter<'a> {
    bs: &'a Bitset,
    idx: usize,
}

impl Bitset {
    pub fn new(len: usize) -> Self {
        Self {
            words: vec![0; (len + 63) / 64],
            len,
        }
    }

    pub fn len(&self) -> usize { self.len }

    // Unsafe optimization potential: Use `slice::get_unchecked` since index has already been
    // verified.
    #[track_caller]
    pub fn get(&self, i: usize) -> bool {
        assert!(i < self.len);
        (self.words[i / 64] & (1 << (i % 64))) != 0
    }
    #[track_caller]
    pub fn set(&mut self, i: usize, value: bool) {
        assert!(i < self.len);
        if value {
            self.words[i / 64] |= 1 << (i % 64);
        } else {
            self.words[i / 64] &= !(1 << (i % 64));
        }
    }

    pub fn count(&self) -> usize {
        self.words.iter().map(|&w| w.count_ones() as usize).sum()
    }

    pub fn none(&self) -> bool {
        self.words.iter().all(|&w| w == 0)
    }
    pub fn any(&self) -> bool {
        self.words.iter().any(|&w| w != 0)
    }
    pub fn all(&self) -> bool {
        self.count() == self.len
    }

    pub fn fill(&mut self, value: bool) {
        self.words.fill(if value { !0 } else { 0 });
        self.fix_last_word();
    }
    pub fn flip(&mut self) {
        for w in self.words.iter_mut() {
            *w = !*w;
        }
        self.fix_last_word();
    }

    pub fn iter(&self) -> BitsetIter {
        BitsetIter {
            bs: &self,
            idx: 0,
        }
    }

    fn fix_last_word(&mut self) {
        let partial_len = self.len % 64;
        if partial_len != 0 {
            self.words.last_mut().map(|w| *w &= (1 << partial_len) - 1);
        }
    }
}

impl<I> From<I> for Bitset
where
    I: ExactSizeIterator<Item = bool>,
{
    fn from(iter: I) -> Self {
        let mut bs = Self::new(iter.len());
        for (i, v) in iter.enumerate() {
            bs.set(i, v);
        }
        bs
    }
}
impl TryFrom<Bitset> for u64 {
    type Error = &'static str;
    fn try_from(bs: Bitset) -> Result<Self, Self::Error> {
        if bs.len > 64 {
            Err("Bitset too large")
        } else {
            Ok(*bs.words.first().unwrap_or(&0))
        }
    }
}

impl ops::BitAndAssign<&Self> for Bitset {
    fn bitand_assign(&mut self, rhs: &Self) {
        assert_eq!(self.len, rhs.len);
        for (a, b) in self.words.iter_mut().zip(rhs.words.iter()) {
            *a &= *b;
        }
    }
}
impl ops::BitOrAssign<&Self> for Bitset {
    fn bitor_assign(&mut self, rhs: &Self) {
        assert_eq!(self.len, rhs.len);
        for (a, b) in self.words.iter_mut().zip(rhs.words.iter()) {
            *a |= *b;
        }
    }
}
impl ops::BitXorAssign<&Self> for Bitset {
    fn bitxor_assign(&mut self, rhs: &Self) {
        assert_eq!(self.len, rhs.len);
        for (a, b) in self.words.iter_mut().zip(rhs.words.iter()) {
            *a ^= *b;
        }
    }
}

impl ops::BitAnd<&Self> for Bitset {
    type Output = Self;
    fn bitand(mut self, rhs: &Self) -> Self {
        self &= rhs;
        self
    }
}
impl ops::BitOr<&Self> for Bitset {
    type Output = Self;
    fn bitor(mut self, rhs: &Self) -> Self {
        self |= rhs;
        self
    }
}
impl ops::BitXor<&Self> for Bitset {
    type Output = Self;
    fn bitxor(mut self, rhs: &Self) -> Self {
        self ^= rhs;
        self
    }
}

impl ops::Not for Bitset {
    type Output = Self;
    fn not(mut self) -> Self {
        self.flip();
        self
    }
}

impl Iterator for BitsetIter<'_> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.bs.len {
            let i = self.idx;
            self.idx += 1;
            Some(self.bs.get(i))
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.bs.len - self.idx;
        (len, Some(len))
    }
}
impl ExactSizeIterator for BitsetIter<'_> {}

impl fmt::Debug for Bitset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bitset {{ ")?;
        for v in self.iter() {
            write!(f, "{}", if v { '1' } else { '0' })?;
        }
        write!(f, " }}")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut bs = Bitset::new(100);
        assert!(!bs.any());
        bs.set(42, true);
        assert!(bs.any());
        assert_eq!(bs.get(41), false);
        assert_eq!(bs.get(42), true);
        assert_eq!(bs.get(43), false);
        assert_eq!(bs.len(), 100);
    }

    #[test]
    fn global_checks() {
        let mut bs = Bitset::new(3);

        assert!(bs.none());
        assert!(!bs.any());
        assert!(!bs.all());

        bs.set(2, true);
        assert!(!bs.none());
        assert!(bs.any());
        assert!(!bs.all());

        bs.set(0, true);
        assert!(!bs.none());
        assert!(bs.any());
        assert!(!bs.all());

        bs.set(1, true);
        assert!(!bs.none());
        assert!(bs.any());
        assert!(bs.all());
    }

    #[test]
    fn binary_operation() {
        let mut bs1 = Bitset::new(200);
        let mut bs2 = Bitset::new(200);
        let mut bs3 = Bitset::new(200);

        bs1.set(1, true);
        bs1.set(100, true);

        bs2.set(1, true);
        bs2.set(100, true);
        bs2.set(101, true);
        bs2.set(199, true);

        bs3.set(101, true);
        bs3.set(199, true);

        let mut bs1_and_bs2 = bs1.clone();
        bs1_and_bs2 &= &bs2;
        let mut bs1_or_bs2 = bs1.clone();
        bs1_or_bs2 |= &bs2;
        let mut bs1_xor_bs2 = bs1.clone();
        bs1_xor_bs2 ^= &bs2;

        assert_eq!(bs1.clone() & &bs2, bs1_and_bs2);
        assert_eq!(bs1.clone() | &bs2, bs1_or_bs2);
        assert_eq!(bs1.clone() ^ &bs2, bs1_xor_bs2);

        assert_eq!(bs1_and_bs2, bs1);
        assert_eq!(bs1_or_bs2, bs2);
        assert_eq!(bs1_xor_bs2, bs3);
    }

    #[test]
    fn last_word() {
        for n in [251, 255, 256] {
            let mut bs_manual = Bitset::new(n);
            let mut bs_fill = Bitset::new(n);
            let mut bs_operations = Bitset::new(n);

            for i in 0..n {
                bs_manual.set(i, true);
            }
            bs_fill.fill(true);

            bs_operations.set(3, true);
            bs_operations.set(5, true);
            bs_operations.set(77, true);
            bs_operations.set(250, true);
            bs_operations = bs_operations.clone() | &!bs_operations.clone();

            assert_eq!(bs_manual, bs_fill);
            assert_eq!(bs_manual, bs_operations);

            assert_eq!(bs_manual.count(), n);
            assert_eq!(bs_fill.count(), n);
            assert_eq!(bs_operations.count(), n);
        }
    }

    #[test]
    fn iteration() {
        let mut bs = Bitset::new(10);
        bs.set(1, true);
        bs.set(4, true);
        bs.set(5, true);
        bs.set(7, true);

        let v: Vec<_> = bs.iter().map(|b| b as i32).collect();
        assert_eq!(v, vec![0, 1, 0, 0, 1, 1, 0, 1, 0, 0]);
    }
}
