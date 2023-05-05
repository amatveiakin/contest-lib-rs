#![cfg(test)]

use crate::io;


pub fn run_solver(
    solve: impl Fn(&mut io::Reader<std::io::Cursor<Vec<u8>>>, &mut std::io::Cursor<Vec<u8>>),
    input: &str,
) -> String {
    let mut read = io::Reader::new(std::io::Cursor::new(input.to_owned().into_bytes()));
    let mut write = std::io::Cursor::new(vec![]);
    solve(&mut read, &mut write);
    String::from_utf8(write.get_ref().clone()).unwrap()
}

pub fn trim_lines(s: &str) -> String {
    s.lines().map(|line| line.trim()).collect::<Vec<_>>().join("\n")
}

#[macro_export]
macro_rules! assert_trimmed_eq {
    ( $left:expr, $right:expr ) => {
        assert_eq!(crate::testing::trim_lines($left), crate::testing::trim_lines($right));
    };
}


#[cfg(test)]
mod tests {
    use crate::io::*;
    use super::*;

    fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
        let n = read.usize();
        let v = read.vec_i32(n);
        emitln!(write, n);
        emitln!(write, v.iter().map(|x| x + 1).collect::<Vec<_>>());
    }

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "3\n1 -2 3\n"), "3\n2 -1 4\n");
    }
}
