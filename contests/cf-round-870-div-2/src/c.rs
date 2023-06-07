use contest_lib_rs::{io, emitln};

fn get_primes() -> Vec<u32> {
    let mut primes = vec![2];
    let mut n = 3;
    while n <= 1000 {
        if primes.iter().all(|&p| n % p != 0) {
            primes.push(n);
        }
        n += 2;
    }
    primes
}

fn smallest_factor(n: u32, primes: &[u32]) -> u32 {
    for &p in primes {
        if p * p > n {
            break;
        }
        if n % p == 0 {
            return p;
        }
    }
    return n;
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let primes = get_primes();
    let t = read.usize();
    for _ in 0..t {
        let n = read.u32();
        let m = read.u32();
        let p = smallest_factor(n, &primes);
        if m < p || p == 1 {
            emitln!(write, "YES");
        } else {
            emitln!(write, "NO");
        }
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
    use pretty_assertions::assert_eq;
    use contest_lib_rs::{solution_testing::run_solver, assert_trimmed_eq};

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        5
        3 2
        4 2
        5 3
        1000000 1000000
        1 1000000
        "), "\
        YES
        NO
        YES
        NO
        YES");
        assert_trimmed_eq!(&run_solver(solve, "\
        3
        9 2
        9 3
        9 4
        "), "\
        YES
        NO
        NO");
    }
}
