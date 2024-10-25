use contest_lib_rs::base_one::{Base, BaseOneConversion};
use contest_lib_rs::bfs::bfs_path;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::tree::Tree;

struct State {
    ow: u32,
    tw: u32,
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let [a, b] = read.usizes().from1b();
    let t = Tree::from_read_edges(n, Base::ONE, read);

    let p = bfs_path(&t, a, b).unwrap();
    let pl = p.len();
    let r = p[(pl - 1) / 2];
    let init_steps = (pl / 2) as u32;

    let t = t.chroot(r);
    let s = t.compute_bottom_up(|ch: &[&State], v| {
        let tw = ch.iter().map(|&c| c.tw + 2).sum::<u32>();
        let mut ow = tw;
        if !ch.is_empty() {
            ow -= ch.iter().map(|&c| (c.tw + 2) - (c.ow + 1)).max().unwrap();
        };
        State { ow, tw }
    });
    emitln!(write, init_steps + s[t.root()].ow);
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
        assert_trimmed_eq!(&run_solver(solve_case, "5   1 5   1 2  2 3  3 4  4 5"), "8");
        assert_trimmed_eq!(&run_solver(solve_case, "4   1 4   1 2  2 3  3 4"), "6");
        assert_trimmed_eq!(&run_solver(solve, "\
        3
        2
        1 2
        1 2
        5
        1 2
        1 2
        1 3
        1 4
        1 5
        8
        5 4
        7 1
        1 5
        1 8
        8 3
        7 2
        8 6
        3 4
        "), "\
        2
        8
        13");
    }
}
