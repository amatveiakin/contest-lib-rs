use std::collections::{HashMap, HashSet};

use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m, k] = read.u32s();
    let mut pairs = HashSet::new();
    let mut xy: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut yx: HashMap<u32, Vec<u32>> = HashMap::new();
    for _ in 0..k {
        let [x, y] = read.u32s();
        pairs.insert((x, y));
        xy.entry(x).or_default().push(y);
        yx.entry(y).or_default().push(x);
    }
    let mut x = n;
    let mut y = m;
    'outer: while x > 0 && y > 0 {
        if pairs.contains(&(x, y)) {
            x -= 1;
            y -= 1;
            continue 'outer;
        }
        if let Some(vy) = xy.get(&x) {
            for &sy in vy {
                if sy > y {
                    x -= 1;
                    continue 'outer;
                }
            }
        }
        if let Some(vx) = yx.get(&y) {
            for &sx in vx {
                if sx > x {
                    y -= 1;
                    continue 'outer;
                }
            }
        }
        x -= 1;
        y -= 1;
    }
    if x == y {
        emitln!(write, "Bhinneka");
    } else {
        emitln!(write, "Chaneka");
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
        assert_trimmed_eq!(&run_solver(solve, "\
        4 5 3
        1 3
        4 4
        1 5
        "), "Chaneka");
        assert_trimmed_eq!(&run_solver(solve, "\
        2 2 0
        "), "Bhinneka");
        assert_trimmed_eq!(&run_solver(solve, "\
        6 6 2
        3 3
        6 4
        "), "Bhinneka");
        assert_trimmed_eq!(&run_solver(solve, "\
        8 11 5
        8 9
        8 8
        8 7
        6 10
        5 10
        "), "Chaneka");
        assert_trimmed_eq!(&run_solver(solve, "\
        4 5 1
        4 2
        "), "Bhinneka");
    }
}
