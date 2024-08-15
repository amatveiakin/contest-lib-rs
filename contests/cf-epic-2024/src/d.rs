// UNFINISHED

use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::bool_ext::BoolExtension;
use contest_lib_rs::io::prelude::*;

fn is_good_element(p: &[usize], v: usize, k: u32) -> bool {
    // if (p[v] + 1).ilog2() != (v + 1).ilog2() {
    //     return false;
    // }

    let l = (p[v] + 1).ilog2();
    println!("p = {p:?}, v = {v}, k = {k}, l = {l}");
    if (v as i32 - l as i32) % 2i32.pow(k - l - 1) != 0 {
        return false;
    }

    // while v != 0 {
    //     let w = (v - 1) / 2;
    //     if p[v] != w {
    //         return false;
    //     }
    //     v = w;
    // }
    true
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, q] = read.usizes();
    let a = read.vec_usize(n - 1).from1b();
    let mut p = read.vec_usize(n).from1b();
    let k = (n + 1).ilog2();
    assert_eq!(n, 2usize.pow(k) - 1);
    for _ in 0..q {
        println!();
        // TODO: Keep count
        let [x, y] = read.usizes().from1b();
        p.swap(x, y);
        emitln!(write, (0..n).all(|i| is_good_element(&p, i, k)).YESNO());
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
    // use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        // assert_trimmed_eq!(&run_solver(solve_case, "\
        // 3 3
        // 1 1
        // 1 2 3
        // 2 3
        // 3 2
        // 1 3"), "\
        // YES
        // YES
        // NO");
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 2
        // 3 3
        // 1 1
        // 1 2 3
        // 2 3
        // 3 2
        // 1 3
        // 7 4
        // 1 1 2 2 3 3
        // 1 2 3 4 5 6 7
        // 3 5
        // 2 5
        // 3 7
        // 4 6"), "\
        // YES
        // YES
        // NO
        // YES
        // NO
        // NO
        // YES");
    }
}
