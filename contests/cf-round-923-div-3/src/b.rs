use std::collections::HashMap;

use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u32(n);

    let mut cnt = HashMap::new();
    for ch in 'a'..='z' {
        cnt.insert(ch, 0);
    }

    let mut ans = String::new();
    for x in a {
        for ch in 'a'..='z' {
            if cnt[&ch] == x {
                ans.push(ch);
                *cnt.get_mut(&ch).unwrap() += 1;
                break;
            }
        }
    }
    emitln!(write, ans);
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
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 5
        // 11
        // 0 0 0 1 0 2 0 3 1 1 4
        // 10
        // 0 0 0 0 0 1 0 1 1 0
        // 1
        // 0
        // 8
        // 0 1 2 3 4 5 6 7
        // 8
        // 0 0 0 0 0 0 0 0
        // "), "\
        // abracadabra
        // codeforces
        // a
        // aaaaaaaa
        // dijkstra");
    }
}
