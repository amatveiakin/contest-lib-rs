use contest_lib_rs::base_one::Base;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::tree::Tree;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let t = Tree::from_read_edges(n, Base::ONE, read).unwrap();
    let s = read.word_as_chars();
    assert_eq!(s.len(), n);
    assert_eq!(t.root(), 0);

    let mut l0 = 0;
    let mut l1 = 0;
    let mut lq = 0;
    let mut lqi = 0;
    for i in 1..n {
        if t.children(i).len() == 0 {
            match s[i] {
                '0' => l0 += 1,
                '1' => l1 += 1,
                '?' => lq += 1,
                _ => unreachable!(),
            }
        } else {
            if s[i] == '?' {
                lqi += 1;
            }
        }
    }

    let r = s[0];
    let mut ans;
    if r == '?' {
        if l0 > l1 {
            ans = l0;
        } else {
            ans = l1;
        }
        if l0 == l1 {
            ans += (lq + (lqi % 2)) / 2;
        } else {
            ans += lq / 2;
        }
    } else {
        if r == '0' {
            ans = l1;
        } else {
            ans = l0;
        }
        ans += (lq + 1) / 2;
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
        assert_trimmed_eq!(&run_solver(solve, "\
        6
        4
        1 2
        1 3
        4 1
        0101
        4
        1 2
        3 2
        2 4
        ???0
        5
        1 2
        1 3
        2 4
        2 5
        ?1?01
        6
        1 2
        2 3
        3 4
        5 3
        3 6
        ?0????
        5
        1 2
        1 3
        1 4
        1 5
        11?1?
        2
        2 1
        ??"), "\
        2
        1
        1
        2
        1
        0");
    }
}
