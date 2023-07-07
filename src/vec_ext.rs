// Iterated vector constructor. Similar to `vec![val; len]` but the value is computed for each
// element. Example:
//    let v = ivec![None; n];  // works for Option<T> where T is not Clone
#[macro_export]
macro_rules! ivec {
    [$val:expr; $len:expr] => {
        (0..$len).map(|_| $val).collect::<Vec<_>>()
    };
}
