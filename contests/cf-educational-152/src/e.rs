// UNFINISHED

use contest_lib_rs::{io, emitln};
use contest_lib_rs::segment_tree::{new_min_tree, new_max_tree};

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let p = read.vec_i32(n);
    let mut pos = vec![1_000_000_000; n + 1];
    for i in 0..n {
        pos[p[i] as usize] = i as u32;
    }
    let mut min_tree = new_min_tree(&p);
    let mut max_tree = new_max_tree(&p);
    let mut l = 0 as u32;
    let mut r = n as u32;
    let mut answer = 0;
    while l < r {
        let min_pos = pos[min_tree.get(l..r) as usize];
        let max_pos = pos[max_tree.get(l..r) as usize];
        if max_pos > min_pos {
            while l <= min_pos {
                answer += r - l - 1;
                l += 1;
            }
            while r > max_pos {
                answer += r - l - 1;
                r -= 1;
            }
            // l = min_pos + 1;
            // r = max_pos;
            // ...
        } else {
            l = max_pos + 1;
            r = min_pos;
        }
    }
    emitln!(write, answer);
}

fn main() {
    let mut read = io::Reader::new(std::io::stdin().lock());
    let mut write = std::io::stdout().lock();
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
        // assert_trimmed_eq!(&run_solver(solve, "3  1 2 3"), "3");
        // assert_trimmed_eq!(&run_solver(solve, "6  5 3 6 1 4 2"), "4");
        // assert_trimmed_eq!(&run_solver(solve, "10  5 1 6 2 8 3 4 10 9 7"), "38");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
