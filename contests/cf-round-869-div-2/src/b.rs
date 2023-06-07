use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    // 'size: for n in 1..=100 {
    //     'perm: for p in (1..=n).permutations(n) {
    //         for l in 0..n {
    //             for r in l+1..n {
    //                 let s: usize = p[l..=r].iter().sum();
    //                 if s % (r-l+1) == 0 {
    //                     continue 'perm;
    //                 }
    //             }
    //         }
    //         emitln!(write, n, ":", p);
    //         continue 'size;
    //     }
    //     emitln!(write, n, ":", "no");
    // }

    let t = read.usize();
    for _ in 0..t {
        let n = read.i32();
        if n == 1 {
            emitln!(write, 1);
        } else if n % 2 == 0 {
            let p = (1..=n).map(|i| if i % 2 == 0 { i - 1 } else { i + 1 }).collect::<Vec<_>>();
            emitln!(write, p);
        } else {
            emitln!(write, -1);
        }
    }
}

fn main() {
    let mut read = io::Reader::new(std::io::stdin().lock());
    let mut write = std::io::stdout().lock();
    solve(&mut read, &mut write);
}


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use pretty_assertions::assert_eq;
    use contest_lib_rs::{solution_testing::run_solver, assert_trimmed_eq};

    #[test]
    fn test() {
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 3
        // 1
        // 2
        // 3
        // "), "\
        // 1
        // 1 2
        // -1");
        assert_trimmed_eq!(&run_solver(solve, "\
        3
        1
        2
        3
        "), "\
        1
        2 1
        -1");
    }
}
