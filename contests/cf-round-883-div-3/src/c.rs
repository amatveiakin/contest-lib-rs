use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let m = read.usize();
    let h = read.i64();
    let mut results = vec![];
    for _ in 0..n {
        let mut times = read.vec_i64(m);
        times.sort();
        let mut problems = 0;
        let mut total_t = 0;
        let mut penalty = 0;
        for &t in &times {
            total_t += t;
            if total_t <= h {
                problems += 1;
                penalty += total_t;
            } else {
                break;
            }
        }
        results.push((problems, penalty));
    }
    let rudolf = results[0];
    results.sort_by_key(|&(p, t)| (-p, t));
    let pos = results.iter().position(|&x| x == rudolf).unwrap() + 1;
    emitln!(write, pos);
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
        5
        3 3 120
        20 15 110
        90 90 100
        40 40 40
        2 1 120
        30
        30
        1 3 120
        10 20 30
        3 2 27
        8 9
        10 7
        10 8
        3 3 15
        7 2 6
        7 5 4
        1 9 8"), "\
        2
        1
        1
        2
        1");
    }
}
