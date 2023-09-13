pub fn is_zst<T>() -> bool {
    std::mem::size_of::<T>() == 0
}
