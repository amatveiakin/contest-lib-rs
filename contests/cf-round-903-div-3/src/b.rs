use contest_lib_rs::io::prelude::*;

fn apply(v: u32, cuts: u32) -> Option<u32> {
    if v % (cuts + 1) == 0 {
        Some(v / (cuts + 1))
    } else {
        None
    }
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [a, b, c] = read.u32s();
    for x in 0..=3 {
        for y in 0..=3 {
            for z in 0..=3 {
                if x + y + z > 3 {
                    continue;
                }
                if apply(a, x).is_some() && apply(a, x) == apply(b, y) && apply(b, y) == apply(c, z) {
                    emitln!(write, "YES");
                    return;
                }
            }
        }
    }
    emitln!(write, "NO");
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
        15
        1 3 2
        5 5 5
        6 36 12
        7 8 7
        6 3 3
        4 4 12
        12 6 8
        1000000000 1000000000 1000000000
        3 7 1
        9 9 1
        9 3 6
        2 8 2
        5 3 10
        8 4 8
        2 8 4
        "), "\
        YES
        YES
        NO
        NO
        YES
        YES
        NO
        YES
        NO
        NO
        YES
        YES
        NO
        YES
        NO");
    }
}
