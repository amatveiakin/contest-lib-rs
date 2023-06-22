pub fn sort_array<T: Ord, const N: usize>(mut a: [T; N]) -> [T; N] {
    a.sort();
    a
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_usage() {
        let x = 7;
        let y = 5;
        let z = 11;
        let [x, y, z] = sort_array([x, y, z]);
        assert_eq!(x, 5);
        assert_eq!(y, 7);
        assert_eq!(z, 11);
    }
}
