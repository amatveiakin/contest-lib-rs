use crate::io::prelude::*;

use super::io_utils::reader_from_string;


pub mod prelude {
    pub use crate::assert_trimmed_eq;
    pub use crate::assert_fuzzy_eq;
    pub use super::run_solver;
}

pub fn run_solver(
    solve: impl Fn(&mut Reader<std::io::Cursor<Vec<u8>>>, &mut std::io::Cursor<Vec<u8>>),
    input: &str,
) -> String {
    let mut read = reader_from_string(input);
    let mut write = std::io::Cursor::new(vec![]);
    solve(&mut read, &mut write);
    String::from_utf8(write.get_ref().clone()).unwrap()
}

pub fn trim_lines(s: &str) -> String {
    s.trim().lines().map(|line| line.trim()).collect::<Vec<_>>().join("\n")
}

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Word(String),
    Int(i64),
    Float(f64),
}

pub fn to_tokens(s: &str) -> Vec<Token> {
    s.split_ascii_whitespace()
        .map(|x| {
            if let Ok(i) = x.parse::<i64>() {
                Token::Int(i)
            } else if let Ok(f) = x.parse::<f64>() {
                Token::Float(f)
            } else {
                Token::Word(x.to_string())
            }
        })
        .collect()
}

fn fuzzy_eq(x: f64, y: f64) -> bool {
    (x - y).abs() / x.min(y).max(1.0) < 1e-6
}

fn tokens_fuzzy_eq(a: &Token, b: &Token) -> bool {
    use Token::*;
    match (a, b) {
        (Float(x), Float(y)) => fuzzy_eq(*x, *y),
        (Float(x), Int(y)) => fuzzy_eq(*x, *y as f64),
        (Int(x), Float(y)) => fuzzy_eq(*x as f64, *y),
        (_, _) => a == b,
    }
}

pub fn token_streams_fuzzy_eq(a_vec: &[Token], b_vec: &[Token]) -> bool {
    if a_vec.len() != b_vec.len() {
        return false;
    }
    for (a, b) in a_vec.into_iter().zip(b_vec) {
        if !tokens_fuzzy_eq(a, b) {
            return false;
        }
    }
    true
}

#[macro_export]
macro_rules! assert_trimmed_eq {
    ( $left:expr, $right:expr ) => {
        assert_eq!(
            $crate::testing::solution_testing::trim_lines($left),
            $crate::testing::solution_testing::trim_lines($right)
        );
    };
}

#[macro_export]
macro_rules! assert_fuzzy_eq {
    ( $left:expr, $right:expr ) => {{
        let left = $left;
        let right = $right;
        assert!(
            $crate::testing::solution_testing::token_streams_fuzzy_eq(
                & $crate::testing::solution_testing::to_tokens(left),
                & $crate::testing::solution_testing::to_tokens(right)
            ),
            "\n{}\n!=\n{}\n",
            $crate::testing::solution_testing::trim_lines(left),
            $crate::testing::solution_testing::trim_lines(right),
        );
    }};
}


#[cfg(test)]
mod tests {
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
