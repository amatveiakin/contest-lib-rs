use contest_lib_rs::{io, emitln};

fn solve_pos(a: &[i32]) -> Vec<(usize, usize)> {
    let n = a.len();
    if n == 1 {
        return vec![];
    }
    let maxabs = a.iter().enumerate().max_by_key(|&(_, x)| x).unwrap().0;
    assert!(a[maxabs] >= 0);
    let mut ret = vec![];
    ret.push((1, maxabs));
    ret.push((1, maxabs));
    ret.push((1, maxabs));
    ret.push((1, maxabs));
    let mut i = 2;
    while i < n - 1 {
        ret.push((i + 1, i - 1));
        ret.push((i + 1, i - 1));
        if a[i] >= 0 {
            ret.push((i, i - 1));
        } else {
            ret.push((i, i + 1));
        }
        i += 2;
    }
    if i < n {
        ret.push((i, i - 1));
        ret.push((i, i - 1));
    }
    ret
}

fn solve_impl(a: &[i32]) -> Vec<(usize, usize)> {
    let n = a.len();
    let maxabs = a.iter().enumerate().max_by_key(|&(_, x)| x.abs()).unwrap().0;
    if a[maxabs] >= 0 {
        solve_pos(a)
    } else {
        let mut a = a.into_iter().map(|x| -x).collect::<Vec<_>>();
        a.reverse();
        let ret = solve_pos(&a);
        ret.into_iter().map(|(a, b)| (n - a - 1, n - b - 1)).collect::<Vec<_>>()
    }
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_i32(n);
    let ret = solve_impl(&a);
    emitln!(write, ret.len());
    for (a, b) in ret {
        emitln!(write, a + 1, b + 1);
    }
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let t = read.usize();
    for _ in 0..t {
        solve_case(read, write);
    }
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
    use contest_lib_rs::iterutils_is_sorted::IterutilsIsSorted;
    use contest_lib_rs::rand::random;
    use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[track_caller]
    fn solve_and_verify(a: &[i32]) {
        let ret = solve_impl(a);
        assert!(ret.len() <= 31);
        let mut b = a.to_vec();
        for &(i, j) in &ret {
            b[i] += b[j];
        }
        assert!(b.iter().issorted(), "\na = {a:?}\nret = {ret:?}\nb = {b:?}\n");
    }

    #[test]
    fn test() {
        solve_and_verify(&[2, 1]);
        solve_and_verify(&[1, 2, -10, 3]);
        solve_and_verify(&[2, 1, 1, 1, 1]);
        solve_and_verify(&[0, 0, 0, 0, 0, 0, 0, 0]);
        solve_and_verify(&[1, 2, -4, 3, -10]);
        solve_and_verify(&[11, 12, 13, 14, 15, -15, -16, -17, -18, -19]);
        solve_and_verify(&[1, 9, 3, -4, -3, -2, -1]);
        solve_and_verify(&[10, 9, 8]);
        solve_and_verify(&[1, -14, 2, -10, 6, -5, 10, -13, 10, 7, -14, 19, -5, 19, 1, 18, -16, -7, 12, 8]);
        solve_and_verify(&[-15, -17, -13, 8, 14, -13, 10, -4, 11, -4, -16, -6, 15, -4, -2, 7, -9, 5, -5, 17]);

        solve_and_verify(&[1]);
        solve_and_verify(&[1, 1]);
        solve_and_verify(&[0]);
        solve_and_verify(&[-1]);
        solve_and_verify(&[-1, -1]);
        solve_and_verify(&[1, 2, 3]);
        solve_and_verify(&[1, -1, -1, 1, -1, -1]);
        solve_and_verify(&[1, -20, 1]);
        solve_and_verify(&[1, 1, -20, 1, 1]);
        solve_and_verify(&[-1, 20, -1]);
        solve_and_verify(&[-1, -1, 20, -1, -1]);
        solve_and_verify(&[20, 1, 1, 1, 1]);
        solve_and_verify(&[1, 1, 1, 1, 20]);

        // loop {
        //     let n: u32 = random::<u32>() % 20 + 1;
        //     let mut a = vec![];
        //     for _ in 0..n {
        //         a.push(random::<i32>().rem_euclid(41) - 20);
        //     }
        //     solve_and_verify(&a);
        // }

        // assert_trimmed_eq!(&run_solver(solve, ""), "");
        // assert_trimmed_eq!(&run_solver(solve, "1  2  2 1"), "");
    }
}
