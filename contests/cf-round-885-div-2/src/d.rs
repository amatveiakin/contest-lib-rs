use contest_lib_rs::{io, emitln};
use contest_lib_rs::relax::RelaxMinMax;

fn disc(mut s: i64, k: i64, mut inc: i64) -> i64 {
    let apply: i64 = k - inc;
    if inc < 0 || apply < 0 {
        return 0;
    }
    if inc > 0 {
        s += s % 10;
        inc -= 1;
        if s % 10 != 0 {
            s += 20 * (inc / 4);
            inc %= 4;
            while inc > 0 {
                s += s % 10;
                inc -= 1;
            }
        }
    }
    s * apply
}

// #[allow(unused_variables)]
// fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
//     let [s, k] = read.i64s();
//     let mut l = 0;
//     let mut r = k;
//     while l + 10 < r {
//         let ml = (2 * l + r) / 3;
//         let mr = (l + 2 * r) / 3;
//         let vml = disc(s, k, ml);
//         let vmr = disc(s, k, mr);
//         if vml > vmr {
//             r = mr;
//         } else {
//             l = ml;
//         }
//     }
//     let mut answer = 0;
//     l = (l - 20).max(0);
//     r = (r + 20).min(k);
//     for m in l..=r {
//         let v = disc(s, k, m);
//         answer.relax_max(v);
//     }
//     let v = disc(s, k, 0);
//     answer.relax_max(v);
//     emitln!(write, answer);
// }

// #[allow(unused_variables)]
// fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
//     let [s, k] = read.i64s();
//     let mut answer = 0;
//     let mut best = -1;
//     for m in 0..=k {
//         let v = disc(s, k, m);
//         if v > answer {
//             answer = v;
//             best = m;
//         }
//     }
//     eprintln!("{}", best);
//     emitln!(write, answer);
// }

// #[allow(unused_variables)]
// fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
//     let [s, k] = read.i64s();
//     let mut answer = 0;
//     let mut best = -1;
//     let mut prev = 0;
//     // for m in 0..=k {
//     for m in 0..100 {
//         let v = disc(s, k, m);
//         if m % 4 == 0 {
//             eprintln!("d = {}", v - prev);
//             prev = v;
//         }
//         // eprintln!("d = {}", v - prev);
//         if v > answer {
//             answer = v;
//             best = m;
//         }
//     }
//     eprintln!("{}", best);
//     emitln!(write, answer);
// }

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let [s, k] = read.i64s();
    let mut answer = 0;
    for rem in 0..4 {
        let mut l = 0;
        let mut r = k / 4;
        while l + 5 < r {
            let ml = (2 * l + r) / 3;
            let mr = (l + 2 * r) / 3;
            let vml = disc(s, k, ml * 4 + rem);
            let vmr = disc(s, k, mr * 4 + rem);
            if vml > vmr {
                r = mr;
            } else {
                l = ml;
            }
        }
        l = ((l - 2) * 4).max(0);
        r = ((r + 2) * 4).min(k);
        for m in l..=r {
            let v = disc(s, k, m);
            answer.relax_max(v);
        }
    }
    let v = disc(s, k, 0);
    answer.relax_max(v);
    emitln!(write, answer);
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let t = read.usize();
    for _ in 0..t {
        solve_case(read, write);
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
    // use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        6
        1 3
        11 3
        0 179
        5 1000000000
        723252212 856168102
        728598293 145725253
        "), "\
        4
        33
        0
        9999999990
        1252047198518668448
        106175170582793129");
    }
}
