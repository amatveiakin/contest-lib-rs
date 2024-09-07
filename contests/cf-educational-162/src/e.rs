// UNFINISHED

use contest_lib_rs::base_one::{Base, BaseOneConversion};
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::tree::Tree;

fn dfs(
    t: &Tree<(), ()>, v: usize, c: &Vec<usize>,
    cin: &mut Vec<u32>, cout: &mut Vec<u32>, ans: &mut u64
) {
    if cin[c[v]] - cout[c[v]] > 0 {
        *ans += 1;
    }
    cin[c[v]] += 1;
    for ch in t.children(v) {
        dfs(t, ch, c, cin, cout, ans);
    }
    cout[c[v]] += 1;
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let c = read.vec_usize(n).from1b();
    let t = Tree::from_read_edges(n, Base::ONE, read);
    let mut cnt = vec![0; n];
    let mut cout = vec![0; n];
    let mut ans = 0;
    dfs(&t, t.root(), &c, &mut cnt, &mut cout, &mut ans);
    emitln!(write, ans);
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
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 4
        // 3
        // 1 2 1
        // 1 2
        // 2 3
        // 5
        // 2 1 2 1 2
        // 1 2
        // 1 3
        // 3 4
        // 4 5
        // 5
        // 1 2 3 4 5
        // 1 2
        // 1 3
        // 3 4
        // 4 5
        // 4
        // 2 2 2 2
        // 3 1
        // 3 2
        // 3 4
        // "), "\
        // 1
        // 3
        // 0
        // 3");
    }
}
