use contest_lib_rs::{io, emitln, segment_tree};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.u32();
    let k = read.u32();
    let mut a = read.vec_i64(n as usize);
    a.sort();

    let mut tree = segment_tree::new_sum_tree(&a);
    let mut answer = 0;
    for i in 0..=k {
        answer = answer.max(tree.get((2 * i) .. (n - k + i)));
    }
    emitln!(write, answer);

    // let get_value = |m| {
    //     let num_min =
    // };
    // let mut l = 0;
    // let mut r = k;
    // while r > l + 1 {
    //     let m = (l + r) / 2;
    // }
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
        6
        5 1
        2 5 1 10 6
        5 2
        2 5 1 10 6
        3 1
        1 2 3
        6 1
        15 22 12 10 13 11
        6 2
        15 22 12 10 13 11
        5 1
        999999996 999999999 999999997 999999998 999999995
        "), "\
        21
        11
        3
        62
        46
        3999999986");
    }
}
