use contest_lib_rs::base_one::{Base, BaseOneConversion};
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::runner::prelude::*;
use contest_lib_rs::tree::Tree;

fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, t] = read.usizes();
    assert_eq!(t, 1);
    let tree = Tree::from_read_edges(n, Base::ONE, read);
    for _ in 0..t {
        let u = read.usize().from1b();
        let tree = tree.chroot(u);
        let win = tree.compute_bottom_up(|ch_win: &[&bool], _| {
            ch_win.iter().any(|&&v| !v)
        });
        emitln!(write, if win[u] { "Ron" } else { "Hermione" });
    }
}

fn main() {
    solver_main(solve);
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
        3 1
        2 3
        3 1
        3"), "Ron");
        assert_trimmed_eq!(&run_solver(solve, "\
        5 1
        1 2
        2 3
        3 4
        4 5
        5"), "Hermione");
        assert_trimmed_eq!(&run_solver(solve, "\
        5 1
        1 2
        1 3
        3 4
        3 5
        1"), "Ron");
    }
}
