use crate::num::Integer;


// Not calling it `mex`, because autocompletion is dominated by stuff like `_mm256_extract_epi16`.
pub fn get_mex<T, I>(values: impl IntoIterator<Item = T, IntoIter = I>) -> T
where
    T: Integer,
    I: ExactSizeIterator<Item = T>,
{
    let iter = values.into_iter();
    let len = iter.len();
    let mut present = vec![false; len];
    for v in iter {
        present.get_mut(v.to_usize()).map(|x| *x = true);
    }
    for (i, &p) in present.iter().enumerate() {
        if !p {
            return T::from_usize(i);
        }
    }
    T::from_usize(len)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(get_mex([] as [u32; 0]), 0);
        assert_eq!(get_mex([1, 2, 3]), 0);
        assert_eq!(get_mex([0]), 1);
        assert_eq!(get_mex([0, 1, 2]), 3);
        assert_eq!(get_mex([3, 2, 0, 77, 1, 5]), 4);
    }
}
