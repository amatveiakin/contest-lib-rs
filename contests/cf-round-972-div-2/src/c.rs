use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::relax::Relax;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    const NCH: usize = 5;
    const CHRS: [char; NCH] = ['n', 'a', 'r', 'e', 'k'];
    let [n, m] = read.usizes();
    let strs = (0..n).map(|_| read.word()).collect_vec();
    let mut bsc = [0i64; NCH];
    for s in strs.iter().rev() {
        let mut sc = [0i64; NCH];
        for stch in 0..NCH {
            let mut chid = stch;
            for c in s.chars() {
                if c == CHRS[chid] {
                    chid = (chid + 1) % NCH;
                    sc[stch] -= 1;
                    if chid == 0 {
                        sc[stch] += NCH as i64 * 2;
                    }
                } else if CHRS.iter().any(|&ch| ch == c) {
                    sc[stch] -= 1;
                }
            }
            sc[stch] += bsc[chid];
        }
        for i in 0..NCH {
            bsc[i].relax_max(sc[i]);
        }
    }
    emitln!(write, bsc[0]);
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
        assert_trimmed_eq!(&run_solver(solve, "\
        4
        5 2
        nn
        aa
        rr
        ee
        kk
        1 5
        narek
        1 4
        nare
        5 7
        nrrarek
        nrnekan
        uuuuuuu
        ppppppp
        nkarekz"), "\
        0
        5
        0
        7");
    }
}
