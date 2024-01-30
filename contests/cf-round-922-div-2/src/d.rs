// UNFINISHED

use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u64(n);

    let mut ls = 0;
    let mut rs: u64 = a.iter().sum();
    while ls < rs {
        let s = (ls + rs) / 2;
        println!("s = {}", s);
        let mut seg: u64 = 0;
        let mut block: u64 = 0;
        let mut ok = true;
        for i in 0..n {
            if seg + a[i] <= s {
                seg += a[i];
            } else {
                block += a[i];
                if block > s {
                    ok = false;
                    break;
                }
                seg = 0;
            }
        }
        if ok {
            rs = s;
        } else {
            ls = s + 1;
        }
    }
    emitln!(write, ls);
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
        // assert_trimmed_eq!(&run_solver(solve_case, "5  1 2 3 4 5"), "5");
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 3
        // 6
        // 1 4 5 3 3 2
        // 5
        // 1 2 3 4 5
        // 6
        // 4 1 6 3 10 7
        // "), "\
        // 7
        // 5
        // 11");
    }
}
