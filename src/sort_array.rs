// Unfortunately, Rust does not geenrate optimal machine code for sorting small arrays:
// https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=3cb9d99a80cabf0651d45c1eec96de89
pub fn sort_array<T: Ord, const N: usize>(mut v: [T; N]) -> [T; N] {
    match N {
        0 | 1 => {},
        2 => {
            if v[1] < v[0] {
                v.swap(0, 1);
            }
        },
        _ => v.sort(),
    }
    v
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
