// `memoize` adds caching to a single-argument recursive function `f`. The resulting object is
// guaranteed to call `f` at most once for each argument. This is useful for dynamic programming.
//
// `f` may capture variables from its environment immutably. If mutable captures are needed, a
// `RefCell` can be used.
//
// The cache is preserved across calls as long as the `memoize` object is alive. There are no size
// or time limits.
//
// Note that `memoize` is slower than a hand-written memoization:
//   - It always relies on a `HashMap`, whereas a `Vec` or an `Array2D` could often be used in
//     practice.
//   - It uses `RefCell` to allow for recursive calls. If the cache were stored separately from the
//     function object, this would not be necessary.
//
// See also: `recurse` for a recursive closure without caching.

use std::collections::HashMap;
use std::hash::Hash;
use std::cell::RefCell;

use crate::callable::Callable;


pub struct Memoizer<T, U, F>
where
    T: Eq + Hash + Clone,
    U: Clone,
    F: Fn(T, &dyn Callable<T, U>) -> U,
{
    func: F,
    cache: RefCell<HashMap<T, U>>,
}

impl<T, U, F> Callable<T, U> for Memoizer<T, U, F>
where
    T: Eq + Hash + Clone,
    U: Clone,
    F: Fn(T, &dyn Callable<T, U>) -> U,
{
    fn call(&self, arg: T) -> U {
        if let Some(val) = self.cache.borrow().get(&arg) {
            return val.clone();
        }
        let result = (self.func)(arg.clone(), self);
        self.cache.borrow_mut().insert(arg, result.clone());
        result
    }
}

pub fn memoize<T, U, F>(func: F) -> Memoizer<T, U, F>
where
    T: Eq + Hash + Clone,
    U: Clone,
    F: Fn(T, &dyn Callable<T, U>) -> U,
{
    Memoizer {
        func,
        cache: RefCell::new(HashMap::new()),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let total_calls = RefCell::new(0);
        let fib = memoize(|x: u64, f| {
            *total_calls.borrow_mut() += 1;
            if x <= 1 {
                x
            } else {
                f.call(x - 1) + f.call(x - 2)
            }
        });
        assert_eq!(fib.call(10), 55);
        assert_eq!(fib.call(30), 832040);
        assert_eq!(fib.call(1), 1);
        assert_eq!(fib.call(10), 55);
        assert_eq!(*total_calls.borrow(), 31);
    }
}
