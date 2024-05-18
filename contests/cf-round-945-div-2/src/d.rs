use contest_lib_rs::io::prelude::*;

fn query<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W, l: i64, x: i64) -> i64 {
    emitln!(write, "?", l, x);
    write.flush().unwrap();
    let ret = read.i64();
    assert!(ret >= 0);
    ret
}

fn answer<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W, m: i64) {
    emitln!(write, "!", m);
    write.flush().unwrap();
    let ret = read.i64();
    assert!(ret == 1);
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, k] = read.i64s();
    let mut max = 0;
    for i in (1..=n).rev() {
        let r = query(read, write, 1, i * n);
        if r <= n {
            assert_eq!(r, n);
            max = i;
            break;
        }
    }
    assert!(max > 0);
    'outer: for i in (1..=(n / k)).rev() {
        let m = i * max;
        let mut p = 0;
        for j in 1..=k {
            if p >= n {
                continue 'outer;
            }
            p = query(read, write, p + 1, m);
        }
        if p == n {
            answer(read, write, m);
            return;
        }
    }
    answer(read, write, -1);
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
    // let mut write = std::io::BufWriter::new(std::io::stdout().lock());
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
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
