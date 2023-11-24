use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::relax::Relax;
use contest_lib_rs::segment_tree::new_max_tree;

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u32(n);
    let l = a.iter().enumerate().map(|(i, &x)| x + (n - i - 1) as u32).collect_vec();
    let r = a.iter().enumerate().map(|(i, &x)| x + i as u32).collect_vec();
    let mut l = new_max_tree(&l);
    let mut r = new_max_tree(&r);
    let mut ans = u32::MAX;
    let n = n as u32;
    for i in 0..n {
        ans.relax_min(a[i as usize].max(l.get(0..i)).max(r.get(i + 1..n)));
    }
    emitln!(write, ans);
}

fn main() {
    let mut read = Reader::new(std::io::stdin().lock());
    let mut write = std::io::BufWriter::new(std::io::stdout().lock());
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
        assert_trimmed_eq!(&run_solver(solve, "6  2 1 5 6 4 3"), "8");
        assert_trimmed_eq!(&run_solver(solve, "5  4 4 4 4 4"), "8");
        assert_trimmed_eq!(&run_solver(solve, "2  1 1000000000"), "1000000000");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
