use contest_lib_rs::base_one::{Base, BaseOneConversion};
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::tree::Tree;

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, t] = read.usizes();
    let tree = Tree::from_read_edges(n, Base::ONE, read);
    let child_win = tree.compute_recursively(|ch_win: &[&bool], _| {
        ch_win.iter().any(|&&v| !v)
    });
    let parent_win = tree.compute_down_recursively(false, |&p_win: &bool, v| {
        !p_win && tree.silblings(v).all(|u| child_win[u])
    });

    for _ in 0..t {
        let u = read.usize().from1b();
        let win = child_win[u] || parent_win[u];
        emitln!(write, if win { "Ron" } else { "Hermione" });
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

        assert_trimmed_eq!(&run_solver(solve, "\
        5 2
        1 2
        1 3
        3 4
        3 5
        1 2"), "\
        Ron
        Ron");
        assert_trimmed_eq!(&run_solver(solve, "\
        6 3
        1 2
        2 3
        1 4
        4 5
        4 6
        1 4 6"), "\
        Hermione
        Ron
        Hermione");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
