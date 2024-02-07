use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let w = read.word_as_chars();
    assert!(w.len() == n);
    let begin = w.iter().position(|&c| c == 'B');
    let end = w.iter().rposition(|&c| c == 'B');
    let len = if let (Some(begin), Some(end)) = (begin, end) {
        end - begin + 1
    } else {
        0
    };
    emitln!(write, len);
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
        8
        6
        WBBWBW
        1
        B
        2
        WB
        3
        BBW
        4
        BWWB
        6
        BWBWWB
        6
        WWBBWB
        9
        WBWBWWWBW
        "), "\
        4
        1
        1
        2
        4
        6
        4
        7");
    }
}
