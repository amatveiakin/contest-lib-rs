use crate::num::RegularInteger;


// TODO: Benchmark if this is faster than `HashSet`.
// Not calling if `mex`, because autocompletion is dominated by stuff like `_mm256_extract_epi16`.
pub fn get_mex<T: RegularInteger>(mut values: Vec<T>) -> T {
    values.sort_unstable();
    let mut mex = T::zero();
    for value in values {
        if value > mex {
            break;
        }
        mex += T::one();
    }
    mex
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(get_mex(Vec::<i32>::new()), 0);
        assert_eq!(get_mex(vec![1, 2, 3]), 0);
        assert_eq!(get_mex(vec![0]), 1);
        assert_eq!(get_mex(vec![0, 1, 2]), 3);
        assert_eq!(get_mex(vec![3, 2, 0, 7, 1, 5]), 4);
    }
}
