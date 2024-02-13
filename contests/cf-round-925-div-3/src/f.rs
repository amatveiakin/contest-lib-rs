use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;

fn satisfies(scr: &[u32], ord: &[u32]) -> bool {
    assert_eq!(scr.len() + 1, ord.len());
    let mut i = 0;
    let mut j = 0;
    let mut bumped = false;
    while i < scr.len() && j < ord.len() {
        if scr[i] != ord[j] {
            if bumped {
                return false;
            }
            j += 1;
            bumped = true;
        } else {
            i += 1;
            j += 1;
        }
    }
    true
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, k] = read.usizes();
    let scrs = (0..k).map(|_| read.vec_u32(n).from1b()).collect_vec();

    if k == 1 {
        emitln!(write, "YES");
        return;
    }

    let s0 = &scrs[0];
    let s1 = &scrs[1];
    let mut ords = None;
    for i in 1..n {
        if s0[i] != s1[i] {
            let mut o1 = vec![];
            o1.extend_from_slice(&s1[1..i]);
            o1.push(s1[0]);
            o1.extend_from_slice(&s1[i..]);

            let mut o2 = vec![];
            o2.extend_from_slice(&s0[1..i]);
            o2.push(s0[0]);
            o2.extend_from_slice(&s0[i..]);

            ords = Some([o1, o2]);
            break;
        }
    }
    let ords = ords.unwrap();

    for o in 0..=1 {
        let mut ok = true;
        for src in &scrs {
            if !satisfies(&src[1..], &ords[o]) {
                ok = false;
            }
        }
        if ok {
            emitln!(write, "YES");
            return;
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
    // use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve_case, "\
        4 4
        1 2 3 4
        2 3 1 4
        3 2 1 4
        4 2 3 1
        "), "YES");

        assert_trimmed_eq!(&run_solver(solve_case, "\
        5 4
        3 5 1 4 2
        2 5 1 4 3
        1 5 4 3 2
        5 1 4 3 2
        "), "YES");

        assert_trimmed_eq!(&run_solver(solve, "\
        10
        5 1
        1 2 3 4 5
        4 4
        1 2 3 4
        2 3 1 4
        3 2 1 4
        4 2 3 1
        6 2
        1 3 5 2 4 6
        6 3 5 2 1 4
        3 3
        1 2 3
        2 3 1
        3 2 1
        10 2
        1 2 3 4 5 6 7 8 9 10
        10 9 8 7 6 5 4 3 2 1
        1 1
        1
        5 2
        1 2 3 5 4
        2 1 3 5 4
        3 3
        3 1 2
        2 3 1
        1 3 2
        5 4
        3 5 1 4 2
        2 5 1 4 3
        1 5 4 3 2
        5 1 4 3 2
        3 3
        1 3 2
        2 1 3
        3 2 1
        "), "\
        YES
        YES
        YES
        YES
        NO
        YES
        YES
        YES
        YES
        NO");
    }
}
