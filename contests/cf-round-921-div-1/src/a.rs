use std::collections::HashSet;

use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, k, m] = read.usizes();
    let s = read.word();
    assert!(s.len() == m);
    let mut found = HashSet::new();
    let mut long = String::new();
    for c in s.chars() {
        if (c as u8) < b'a' + k as u8 {
            found.insert(c);
        }
        if found.len() == k {
            found.clear();
            long.push(c);
        }
    }
    if long.len() >= n {
        emitln!(write, "YES");
    } else {
        emitln!(write, "NO");
        let missing = (0..k as u8).find(|&c| !found.contains(&((b'a' + c) as char))).unwrap();
        while long.len() < n {
            long.push((b'a' + missing) as char);
        }
        emitln!(write, long);
    }
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
        // 3
        // 2 2 4
        // abba
        // 2 2 3
        // abb
        // 3 3 10
        // aabbccabab
        // "), "\
        // YES
        // NO
        // aa
        // NO
        // ccc");
    }
}
