use std::collections::HashMap;

use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m, k] = read.usizes();
    let mut a = read.vec_u64(n);
    let mut b = read.vec_u64(m);
    let mut states: HashMap<(Vec<u64>, Vec<u64>), usize> = HashMap::new();
    for step in 0..k {
        a.sort();
        b.sort();
        let key = (a.clone(), b.clone());
        if step > 2 {
            if let Some(prev) = states.get(&key) {
                let period = step - prev;
                let left = k - step;
                let final_state = states.iter().find(|(_, &v)| v == prev + left % period).unwrap().0;
                emitln!(write, final_state.0.iter().sum::<u64>());
                return;
            } else {
                states.insert(key, step);
            }
        }
        if step % 2 == 0 {
            if a.first().unwrap() < b.last().unwrap() {
                std::mem::swap(a.first_mut().unwrap(), b.last_mut().unwrap())
            }
        } else {
            if b.first().unwrap() < a.last().unwrap() {
                std::mem::swap(b.first_mut().unwrap(), a.last_mut().unwrap())
            }
        }
    }
    emitln!(write, a.iter().sum::<u64>());
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
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve_case, "\
        3 3 2
        10 10 10
        1 1 1
        "), "21");
        assert_trimmed_eq!(&run_solver(solve, "\
        4
        2 2 1
        1 2
        3 4
        1 1 10000
        1
        2
        4 5 11037
        1 1 4 5
        1 9 1 9 8
        1 1 1
        2
        1
        "), "\
        6
        1
        19
        2");
    }
}
