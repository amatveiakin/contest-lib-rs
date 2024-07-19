use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let s = read.word_as_chars();
    let t = read.word_as_chars();
    assert_eq!(s.len(), n);
    assert_eq!(t.len(), n);
    for i in 0..n {
        if s[i] == '1' {
            emitln!(write, "YES");
            return;
        } else if t[i] == '1' {
            emitln!(write, "NO");
            return;
        }
    }
    emitln!(write, "YES");
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let t = read.usize();
    for _ in 0..t {
        solve_case(read, write);
    }
}

fn main() {
    let mut read = Reader::new(std::io::stdin().lock());
    let mut write = std::io::BufWriter::new(std::io::stdout().lock());
    solve(&mut read, &mut write);
}


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        6
        1
        0
        1
        7
        0110100
        0110100
        9
        100101010
        101111110
        4
        0011
        1011
        4
        0100
        0001
        8
        10110111
        01100000
        "), "\
        NO
        YES
        YES
        NO
        YES
        YES");
    }
}
