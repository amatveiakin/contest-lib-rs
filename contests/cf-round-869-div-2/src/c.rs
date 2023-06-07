use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let q = read.usize();
    let a = read.vec_i32(n);
    let mut good = vec![true; n];
    for i in 1..(n-1) {
        good[i] = a[i-1] < a[i] || a[i] < a[i+1];
    }
    let mut num_good = vec![0; n+1];
    for i in 1..(n+1) {
        num_good[i] = num_good[i-1] + if good[i-1] { 1 } else { 0 };
    }

    // eprintln!("{:?}", good);
    // eprintln!("{:?}", num_good);

    for _ in 0..q {
        let l = read.usize() - 1;
        let r = read.usize() - 1;
        // let num = num_good[r+1] - num_good[l];
        // let num = std::cmp::min(std::cmp::max(num, 2), r-l+1);
        if r == l {
            emitln!(write, 1);
            continue;
        }
        let num = num_good[r] - num_good[l+1] + 2;
        emitln!(write, num);
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
        9 8
        1 2 4 3 3 5 6 2 1
        1 3
        1 4
        2 5
        6 6
        3 7
        7 8
        1 8
        8 8
        "), "\
        3
        4
        3
        1
        4
        2
        7
        1");
    }
}
