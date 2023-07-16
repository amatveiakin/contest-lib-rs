use contest_lib_rs::{io, emitln};
use contest_lib_rs::relax::RelaxMinMax;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let [n, k] = read.usizes();
    let cc = read.vec_usize(n);
    let mut pos = vec![0; k];
    let mut steps = vec![vec![]; k];
    for (p, &c) in cc.iter().enumerate() {
        let p = p + 1;
        let c = c - 1;
        steps[c].push(p - pos[c] - 1);
        pos[c] = p;
    }
    for c in 0..k {
        steps[c].push(n - pos[c]);
    }
    let mut answer = usize::MAX;
    for st in steps.iter_mut() {
        st.sort();
        st.reverse();
        let mut a = st[0] / 2;
        if let Some(second) = st.get(1) {
            a.relax_max(st[1]);
        }
        answer.relax_min(a);
    }
    emitln!(write, answer);
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
        5 2
        1 1 2 1 1
        7 3
        1 2 3 3 3 2 1
        6 6
        1 2 3 4 5 6
        8 4
        1 2 3 4 2 3 1 4
        3 1
        1 1 1
        "), "\
        0
        1
        2
        2
        0");
    }
}
