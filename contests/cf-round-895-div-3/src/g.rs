use contest_lib_rs::io::prelude::*;
use contest_lib_rs::prefix_accumulate::{PrefixSum, PrefixProduct};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u64(n);

    if a.iter().all(|&x| x == 1) {
        emitln!(write, 1, 1);
        return;
    }
    let ll = a.iter().position(|&x| x > 1).unwrap();
    let rr = a.iter().rposition(|&x| x > 1).unwrap();
    let a = &a[ll..=rr];

    let mut non1 = vec![];
    let mut total_prod: u64 = 1;
    for (pos, &x) in a.iter().enumerate() {
        if x != 1 {
            non1.push(pos);
        }
        match total_prod.checked_mul(x) {
            None => {
                let l = ll + 1;
                let r = ll + a.len();
                emitln!(write, l, r);
                return;
            }
            Some(pr) => {
                total_prod = pr;
            }
        }
    }

    let sums = PrefixSum::from_iter(a.iter().copied());
    let prods = PrefixProduct::from_iter(a.iter().copied());

    let m = non1.len();
    let mut best_ans = 0;
    let mut best_lr = None;
    for lp in 0..m {
        for rp in lp..m {
            let l = non1[lp] as u32;
            let r = non1[rp] as u32;
            let ans = sums.get(..l) + prods.get(l..=r) + sums.get((r+1)..);
            if ans > best_ans {
                best_ans = ans;
                best_lr = Some((l, r));
            }
        }
    }
    let (l, r) = best_lr.unwrap();
    let l = ll + (l as usize) + 1;
    let r = ll + (r as usize) + 1;
    emitln!(write, l, r);
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
    let mut write = std::io::stdout().lock();
    solve(&mut read, &mut write);
}


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use pretty_assertions::assert_eq;
    use contest_lib_rs::{solution_testing::run_solver, assert_trimmed_eq};

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve_case, "\
        11
        2 1 1 1 1 1 1 1 2 2 2
        "), "9 11");
        assert_trimmed_eq!(&run_solver(solve_case, "\
        11
        2 2 2 1 1 1 1 1 1 1 2
        "), "1 3");
        assert_trimmed_eq!(&run_solver(solve_case, "\
        16
        1 1 2 1 1 1 1 1 1 1 2 2 2 1 1 1
        "), "11 13");
        assert_trimmed_eq!(&run_solver(solve_case, "\
        1
        1
        "), "1 1");
        assert_trimmed_eq!(&run_solver(solve_case, "\
        1
        7
        "), "1 1");
        assert_trimmed_eq!(&run_solver(solve_case, "\
        16
        1 1 100000 1 100000 1 100000 1 100000 1 100000 2 100000 1 1 1
        "), "3 13");

        assert_trimmed_eq!(&run_solver(solve, "\
        9
        4
        1 3 1 3
        4
        1 1 2 3
        5
        1 1 1 1 1
        5
        10 1 10 1 10
        1
        1
        2
        2 2
        3
        2 1 2
        4
        2 1 1 3
        6
        2 1 2 1 1 3
        "), "\
        2 4
        3 4
        1 1
        1 5
        1 1
        1 1
        1 1
        1 1
        1 6");

        // assert_trimmed_eq!(&run_solver(solve, "\
        // 9
        // 4
        // 1 3 1 3
        // 4
        // 1 1 2 3
        // 5
        // 1 1 1 1 1
        // 5
        // 10 1 10 1 10
        // 1
        // 1
        // 2
        // 2 2
        // 3
        // 2 1 2
        // 4
        // 2 1 1 3
        // 6
        // 2 1 2 1 1 3
        // "), "\
        // 2 4
        // 3 4
        // 1 1
        // 1 5
        // 1 1
        // 1 2
        // 2 2
        // 4 4
        // 1 6");
    }
}
