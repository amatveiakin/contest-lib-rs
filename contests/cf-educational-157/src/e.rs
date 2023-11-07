// UNFINISHED

use std::cell::RefCell;

use contest_lib_rs::callable::Callable;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::iterutils_zip_eq::IterutilsZipEq;
use contest_lib_rs::memoize::memoize;
use contest_lib_rs::segment_tree::new_max_tree;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    // const V: u32 = 1_000_001;
    let n = read.usize();
    let m = read.usize();
    let ax = read.vec_u32(n);
    let ay = read.vec_u32(n);
    let bx = read.vec_u32(m);
    let by = read.vec_u32(m);
    let a = ax.into_iter().zip_eq(ay.into_iter()).collect_vec();
    let b = bx.into_iter().zip_eq(by.into_iter()).collect_vec();
    let mut t = vec![new_max_tree(&vec![0; 1_000_000 as usize]); 2];
    for &(x, y) in &a {
        t[0].update(x, &y);
    }
    for &(x, y) in &b {
        t[1].update(x, &y);
    }
    let t = RefCell::new(t);
    let results = memoize(|(pl, cy, d): (u32, u32, u32), f| {
        let ny = (t.borrow_mut())[pl as usize].get(cy..);
        if ny == 0 {
            return -1;
        }
        if d > 1_000 {
            return 0;
        }
        -f.call((1 - pl, ny, d + 1))
    });
    let mut win = 0;
    let mut draw = 0;
    let mut loss = 0;
    for &(x, y) in &a {
        let result = results.call((0, y, 0));
        match result {
            -1 => loss += 1,
            0 => draw += 1,
            1 => win += 1,
            _ => unreachable!(),
        }
    }
    emitln!(write, win, draw, loss);
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
        // 3
        // 8 7 4
        // 7 1 10
        // 2
        // 8 4
        // 5 10
        // 9
        // 8 8 5 5 5 4 4 1 4
        // 2 7 5 2 8 9 7 1 9
        // 10
        // 9 8 7 6 5 5 4 3 2 1
        // 7 1 6 7 5 8 8 4 9 6
        // 1
        // 10
        // 5
        // 1
        // 10
        // 5
        // "), "\
        // 1 1 1
        // 2 4 3
        // 0 1 0");
    }
}
