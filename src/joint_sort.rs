// Implements:
//   * joint_sort, joint_sort_2, joint_sort_3
//   * joint_sort_by_key, joint_sort_by_key_2, joint_sort_by_key_3
//
// These functions take several array of the same length. They sort the first array and apply the
// same permutation to the other arrays.


fn reorder<T>(v: &mut [T], mut indices: Vec<usize>) {
    assert_eq!(v.len(), indices.len());
    for i in 0..v.len() {
        // SAFETY: The `indices` vector in always a permutation of 0, 1, ..., v.len() - 1, because
        // `slice::sort_by_key` guarantees that "all original elements will remain in the slice":
        // https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_key.
        unsafe {
            let x = std::ptr::read(&v[i]);
            let mut j = i;
            loop {
                let k = indices[j];
                indices[j] = j;
                if k == i {
                    break;
                }
                std::ptr::write(&mut v[j], std::ptr::read(&v[k]));
                j = k;
            }
            std::ptr::write(&mut v[j], x);
        }
    }
}

macro_rules! define_joint_sort {
    ($name:ident, $($secondary:ident : $Q:ident),+) => {
        pub fn $name<T: Ord, $($Q),+>(primary: &mut [T], $($secondary: &mut [$Q]),+) {
            let mut indices: Vec<_> = (0..primary.len()).collect();
            indices.sort_by_key(|&i| &primary[i]);
            $(reorder($secondary, indices.clone());)+
            reorder(primary, indices);
        }
    };
}

macro_rules! define_joint_sort_by_key {
    ($name:ident, $($secondary:ident : $Q:ident),+) => {
        pub fn $name<K: Ord, F: FnMut(&T) -> K, T, $($Q),+>(primary: &mut [T], mut f: F, $($secondary: &mut [$Q]),+) {
            let mut indices: Vec<_> = (0..primary.len()).collect();
            indices.sort_by_key(|&i| f(&primary[i]));
            $(reorder($secondary, indices.clone());)+
            reorder(primary, indices);
        }
    };
}

define_joint_sort!(joint_sort, v: Q);
define_joint_sort!(joint_sort_2, v1: Q1, v2: Q2);
define_joint_sort!(joint_sort_3, v1: Q1, v2: Q2, v3: Q3);

define_joint_sort_by_key!(joint_sort_by_key, v: Q);
define_joint_sort_by_key!(joint_sort_by_key_2, v1: Q1, v2: Q2);
define_joint_sort_by_key!(joint_sort_by_key_3, v1: Q1, v2: Q2, v3: Q3);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut a = [3, 1, 5, 1, 2, 5, 1, 4];
        let mut b = [30, 10, 50, 11, 20, 51, 12, 40];
        let mut c = ["three", "one", "five", "one", "two", "five", "one", "four"];
        joint_sort_2(&mut a, &mut b, &mut c);
        assert_eq!(a, [1, 1, 1, 2, 3, 4, 5, 5]);
        assert_eq!(b, [10, 11, 12, 20, 30, 40, 50, 51]);
        assert_eq!(c, ["one", "one", "one", "two", "three", "four", "five", "five"]);
    }

    #[test]
    fn non_cloneable() {
        let mut a = [3, 1, 5, 4, 2];
        let mut b = [Box::new(3), Box::new(1), Box::new(5), Box::new(40), Box::new(200)];
        joint_sort(&mut a, &mut b);
        assert_eq!(a, [1, 2, 3, 4, 5]);
        assert_eq!(b, [Box::new(1), Box::new(200), Box::new(3), Box::new(40), Box::new(5)]);
    }

    #[test]
    fn by_key() {
        let mut a = [3, 1, 5, 4, 2];
        let mut b = ["three", "one", "five", "four", "two"];
        joint_sort_by_key(&mut a, |&x| std::cmp::Reverse(x), &mut b);
        assert_eq!(a, [5, 4, 3, 2, 1]);
        assert_eq!(b, ["five", "four", "three", "two", "one"]);
    }
}
