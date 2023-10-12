use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m] = read.usizes();
    let mut x = read.word();
    let s = read.word();
    let mut op = 0;
    while x.len() <= 100 {
        if x.contains(&s) {
            emitln!(write, op);
            return;
        }
        x = format!("{x}{x}");
        op += 1;
    }
    emitln!(write, -1);
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
        assert_trimmed_eq!(&run_solver(solve, "\
        12
        1 5
        a
        aaaaa
        5 5
        eforc
        force
        2 5
        ab
        ababa
        3 5
        aba
        ababa
        4 3
        babb
        bbb
        5 1
        aaaaa
        a
        4 2
        aabb
        ba
        2 8
        bk
        kbkbkbkb
        12 2
        fjdgmujlcont
        tf
        2 2
        aa
        aa
        3 5
        abb
        babba
        1 19
        m
        mmmmmmmmmmmmmmmmmmm
        "), "\
        3
        1
        2
        -1
        1
        0
        1
        3
        1
        0
        2
        5");
    }
}
