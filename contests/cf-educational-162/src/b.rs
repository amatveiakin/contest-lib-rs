use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let k = read.u64();
    let a = read.vec_u64(n);
    let x = read.vec_i64(n);
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[x[i].abs() as usize] += a[i];
    }
    let mut t = 0;
    for i in 0..s.len() {
        t += s[i];
        if t > (i as u64) * k {
            emitln!(write, "NO");
            return;
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
        5
        3 2
        1 2 3
        -1 2 3
        2 1
        1 1
        -1 1
        4 10
        3 4 2 5
        -3 -2 1 3
        5 3
        2 1 3 2 5
        -3 -2 3 4 5
        2 1
        1 2
        1 2
        "), "\
        YES
        NO
        YES
        YES
        NO");
    }
}
