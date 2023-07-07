use contest_lib_rs::{io, emitln};

fn snow(k: i64, iter: u32) -> Option<i64> {
    let mut a = 1 + k;
    let mut kk = k;
    for _ in 1..iter {
        kk = kk.checked_mul(k)?;
        a = a.checked_add(kk)?;
    }
    Some(a)
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.i64();
    const MAX_ITER: u32 = 6;

    for k in 2..n {
        if k.pow(MAX_ITER) > n {
            break;
        }
        if (n - 1) % k != 0 {
            continue;
        }
        let mut a = 1 + k;
        let mut kk = k;
        loop {
            let Some(new_kk) = kk.checked_mul(k) else {
                break;
            };
            kk = new_kk;
            let Some(new_a) = a.checked_add(kk) else {
                break;
            };
            a = new_a;
            if a == n {
                emitln!(write, "YES");
                return;
            } else if a > n {
                break;
            }
        }
    }

    for iter in 2..MAX_ITER {
        let mut l = 2;
        let mut r = n;
        while l < r {
            let k = (l + r) / 2;
            let Some(a) = snow(k, iter) else {
                r = k;
                continue;
            };
            if a == n {
                emitln!(write, "YES");
                return;
            } else if a < n {
                l = k + 1;
            } else {
                r = k;
            }
        }
        let k = l;
        let a = snow(k, iter).unwrap();
        if a == n {
            emitln!(write, "YES");
            return;
        }
    }

    emitln!(write, "NO");
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
    // use pretty_assertions::assert_eq;
    use contest_lib_rs::{solution_testing::run_solver, assert_trimmed_eq};

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        9
        1
        2
        3
        6
        13
        15
        255
        10101
        1000000
        "), "\
        NO
        NO
        NO
        NO
        YES
        YES
        YES
        YES
        NO");
        assert_trimmed_eq!(&run_solver(solve_case, "973160803270656001"), "NO");
    }
}
