use contest_lib_rs::io::prelude::*;
use contest_lib_rs::mex::get_mex;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u32(n);

    let m = get_mex(a.iter().copied());
    let mut f = vec![false; m as usize];
    let mut nf = m;
    for i in 0..n {
        if a[i] < m {
            if !f[a[i] as usize] {
                f[a[i] as usize] = true;
                nf -= 1;
            }
        }
        if nf == 0 {
            if get_mex(a[i + 1..].iter().copied()) == m {
                emitln!(write, 2);
                emitln!(write, 1, i + 1);
                emitln!(write, i + 2, n);
                return;
            }
            break;
        }
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
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 5
        // 2
        // 0 0
        // 5
        // 0 1 2 3 4
        // 8
        // 0 1 7 1 0 1 0 3
        // 3
        // 2 2 2
        // 4
        // 0 1 2 0
        // "), "\
        // 2
        // 1 1
        // 2 2
        // -1
        // 3
        // 1 3
        // 4 5
        // 6 8
        // 3
        // 1 1
        // 2 2
        // 3 3
        // -1");
    }
}
