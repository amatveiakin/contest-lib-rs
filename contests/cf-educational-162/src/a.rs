use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let mut a = read.vec_u32(n);
    let c = a.iter().filter(|&&x| x == 1).count();
    if c <= 1 {
        emitln!(write, 0);
        return;
    }
    let l = a.iter().position(|&x| x == 1).unwrap();
    let mut op = 0;
    'outer: for i in (0..n).rev() {
        if a[i] == 1 {
            if l + c == i + 1 {
                emitln!(write, op);
                return;
            }
            for j in (0..i).rev() {
                if a[j] == 0 {
                    a[i] = 0;
                    a[j] = 1;
                    op += 1;
                    continue 'outer;
                }
            }
            unreachable!();
        }
    }
    unreachable!();
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
        5
        8
        0 1 1 1 0 1 1 0
        6
        0 1 0 0 0 0
        6
        1 1 1 1 1 1
        5
        1 0 1 0 1
        9
        0 1 1 0 0 0 1 1 0
        "), "\
        1
        0
        0
        2
        3");
    }
}
