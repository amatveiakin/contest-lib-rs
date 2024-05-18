use contest_lib_rs::counting_set::CountingSet;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let p = read.vec_u32(n);
    let mut q = p.into_iter().map(|x| n as u32 + 1 - x).collect_vec();
    let one_p = q.iter().position(|&x| x == 1).unwrap();
    let plus_o = one_p % 2;
    let minus_o = 1 - plus_o;
    for i in 0..n {
        if i % 2 == minus_o {
            q[i] -= 1;
            assert!(q[i] > 0);
        }
    }
    let mut values = CountingSet::from_item_iter(q.iter().copied());
    for i in 0..n {
        if i % 2 == plus_o {
            assert!(values.remove(q[i]));
            while values.contains(&q[i]) {
                q[i] += 1;
            }
            values.push(q[i]);
        }
    }
    emitln!(write, q);
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
        // 4
        // 1 2 3 4
        // 4
        // 4 3 1 2
        // 6
        // 6 5 1 4 2 3
        // 8
        // 1 2 4 5 7 6 8 3
        // "), "\
        // 2 4 1 3
        // 3 1 4 2
        // 2 5 1 4 3 6
        // 5 4 8 2 7 1 6 3");
    }
}
