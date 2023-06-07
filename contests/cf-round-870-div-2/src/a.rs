use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.u32();
    let mut a = read.vec_u32(n as usize);
    a.sort();
    for k in 0..=n {
        // let (truthers, liers) = a.split_at((n - k) as usize);
        // let ok = truthers.iter().all(|&v| k >= v) && liers.iter().all(|&v| k < v);
        // if ok {
        //     emitln!(write, k);
        //     return;
        // }
        let split = n - k;
        let x = split == 0 || a[(split - 1) as usize] <= k;
        let y = split == n || a[split as usize] > k;
        if x && y {
            emitln!(write, k);
            return;
        }
    }
    emitln!(write, -1);
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
    use pretty_assertions::assert_eq;
    use contest_lib_rs::{solution_testing::run_solver, assert_trimmed_eq};

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        7
        2
        1 2
        2
        2 2
        2
        0 0
        1
        1
        1
        0
        5
        5 5 3 3 5
        6
        5 3 6 6 3 5
        "), "\
        1
        -1
        0
        -1
        0
        3
        4");
    }
}
