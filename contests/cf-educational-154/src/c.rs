use contest_lib_rs::io::prelude::*;
use contest_lib_rs::relax::Relax;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let commands = read.word();
    let mut largest_sorted = 0;
    let mut smallest_unsorted = None;
    let mut len = 0;
    for cmd in commands.chars() {
        match cmd {
            '+' => {
                len += 1;
            }
            '-' => {
                if len == 0 {
                    emitln!(write, "NO");
                    return;
                }
                len -= 1;
                largest_sorted.relax_min(len);
                if let Some(v) = smallest_unsorted {
                    if len < v {
                        smallest_unsorted = None;
                    }
                }
            }
            '0' => {
                if let Some(v) = smallest_unsorted {
                    if len < v {
                        smallest_unsorted = Some(len);
                    }
                } else {
                    smallest_unsorted = Some(len);
                }
            }
            '1' => {
                largest_sorted = len;
            }
            _ => unreachable!()
        }
        if let Some(smallest_unsorted) = smallest_unsorted {
            if smallest_unsorted <= 1 {
                emitln!(write, "NO");
                return;
            }
            if smallest_unsorted <= largest_sorted {
                emitln!(write, "NO");
                return;
            }
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
    let mut write = std::io::stdout().lock();
    solve(&mut read, &mut write);
}


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    // use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
7
++1
+++1--0
+0
0
++0-+1-+0
++0+-1+-0
+1-+0
"), "\
YES
NO
NO
NO
YES
NO
NO");
    }
}
