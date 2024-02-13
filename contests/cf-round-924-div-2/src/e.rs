// UNFINISHED

use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, x, y, s] = read.u64s();
    let mut c: u64 = 0;
    let mut r = vec![];
    let mut a = x;
    let mut i = 0;
    let mut force = false;
    let mut ans = vec![];
    while i < n {
        let mut b = a + y;
        if c + b > s || force {
            b = a % y;
            force = false;
        }
        if c + b <= s {
            if b >= y && i > 0 {
                r.push((i - 1, c));
            }
            c += b;
            a = b;
            i += 1;
            ans.push(b);
        } else {
            let Some((oi, oc)) = r.pop() else {
                emitln!(write, "NO");
                return;
            };
            ans.drain(oi as usize..);
            i = oi;
            c = oc;
            force = true;
        }
    }
    if c == s {
        emitln!(write, "YES");
        emitln!(write, ans);
    } else {
        emitln!(write, "NO");
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
        // 5 8 3 28
        // 3 5 3 6
        // 9 1 5 79
        // "), "\
        // YES
        // 8 11 2 2 5
        // NO
        // NO");
    }
}
