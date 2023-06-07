use contest_lib_rs::{io, emitln, mod_ring::ModNumber, counting_set::CountingSet};

type ModNum = ModNumber::<998_244_353>;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W, factorials: &[ModNum]) {
    let catalan = |n: u32| {
        let n = n as usize;
        factorials[2 * n] / (factorials[n] * factorials[n + 1])
    };
    let num_seq = |n: u32| {
        if n % 2 == 0 {
            catalan(n / 2)
        } else {
            ModNum::from(0)
        }
    };

    let n = read.usize();
    let k = read.usize();
    // let mut lefts = CountingSet::new();
    // let mut rights = CountingSet::new();
    let mut ends = CountingSet::new();
    for _ in 0..k {
        let l = read.u32() - 1;
        let r = read.u32();
        // lefts.push(l);
        // rights.push(r);
        ends.push((l, 101));
        ends.push((r, 100));
    }

    let mut prev = 0;
    let mut free = 0;
    let mut balance = 0;
    let mut answer = ModNum::from(1);
    while let Some((pos, t)) = ends.pop_first() {
        let old_balance = balance;
        match t {
            101 => balance += 1,
            100 => balance -= 1,
            _ => unreachable!(),
        }
        if old_balance == 0 {
            free += pos - prev;
        } else {
            if pos == prev {
                // noop
            } else {
                answer *= num_seq(pos - prev);
            }
        }
        prev = pos;
    }
    assert_eq!(balance, 0);
    free += n as u32 - prev;
    answer *= num_seq(free);
    emitln!(write, answer);
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let mut factorials = vec![ModNum::from(0); 300_000];
    factorials[0] = ModNum::from(1);
    for i in 1..factorials.len() {
        factorials[i] = factorials[i - 1] * ModNum::from(i as u32);
    }
    let t = read.usize();
    for _ in 0..t {
        solve_case(read, write, &factorials);
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
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 1
        // 4 3
        // 1 4
        // 1 4
        // 1 4
        // "), "140");
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 7
        // 6 0
        // 5 0
        // 8 1
        // 1 3
        // 10 2
        // 3 4
        // 6 9
        // 1000 3
        // 100 701
        // 200 801
        // 300 901
        // 28 5
        // 1 12
        // 3 20
        // 11 14
        // 4 9
        // 18 19
        // 4 3
        // 1 4
        // 1 4
        // 1 4
        // "), "\
        // 5
        // 0
        // 0
        // 4
        // 839415253
        // 140
        // 2");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
