// UNFINISHED

use std::collections::BTreeMap;

use contest_lib_rs::io::prelude::*;

// #[allow(unused_variables)]
// fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
//     let n = read.usize();
//     let a = read.vec_i32(n);
//     let d = read.vec_i32(n);
//     let amin = *a.iter().min().unwrap();
//     let amax = *a.iter().max().unwrap();
//     if amax == 0 {
//         emitln!(write, 0);
//         return;
//     }
//     let mut l = (amin - 1).max(0);  // not possible
//     let mut r = amax;               // possible
//     while l < r - 1 {
//         let m = (l + r) / 2;
//         let mut can = true;
//
//         let mut a = a.clone();
//         let mut has_space = BTreeSet::from_iter((0..n).filter(|&i| a[i] < m));
//         for i in 0..n {
//             let mv_l = (i as i32 - d[i]).max(0) as usize;
//             let mv_r = (i + d[i] as usize).min(n - 1);
//             while a[i] > m {
//                 if let Some(&j) = has_space.range(mv_l..=mv_r).next() {
//                     let mv_amount = (a[i] - m).min(m - a[j]);
//                     assert!(mv_amount > 0);
//                     a[j] += mv_amount;
//                     a[i] -= mv_amount;
//                     if a[j] == m {
//                         has_space.remove(&j);
//                     }
//                 } else {
//                     can = false;
//                     break;
//                 }
//             }
//         }
//
//         if can {
//             r = m;
//         } else {
//             l = m;
//         }
//     }
//     emitln!(write, r);
// }

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_i64(n);
    let d = read.vec_i32(n);
    let amin = *a.iter().min().unwrap();
    let amax = *a.iter().max().unwrap();
    if amax == 0 {
        emitln!(write, 0);
        return;
    }
    let mut l = (amin - 1).max(0);  // not possible
    let mut r = amax;               // possible
    let mut b = vec![BTreeMap::<i32, i64>::new(); n];
    for i in 0..n {
        let left = (i as i32 - d[i]).max(0);
        let right = (i as i32 + d[i]).min(n as i32 - 1);
        *b[left as usize].entry(right).or_default() += a[i] as i64;
    }
    while l < r - 1 {
        let m = (l + r) / 2;
        let mut can = true;

        let mut people = BTreeMap::<i32, i64>::new();
        let mut s: i64 = people.iter().map(|(_, &count)| count as i64).sum();
        for i in 0..n {
            for (&right, &count) in b[i].iter() {
                *people.entry(right).or_default() += count;
                s += count;
            }
            let mut quota = m;
            while quota > 0 {
                if let Some((right, mut count)) = people.pop_first() {
                    let to_move = (count).min(quota);
                    s -= to_move as i64;
                    quota -= to_move;
                    count -= to_move;
                    if count > 0 {
                        *people.entry(right).or_default() += count;
                    }
                } else {
                    can = false;
                    break;
                }
            }
            if people.first_key_value().map(|(right, _)| *right < i as i32).unwrap_or(false) {
                can = false;
                break;
            }
        }
        if s > m {
            can = false;
        }

        if can {
            r = m;
        } else {
            l = m;
        }
    }
    emitln!(write, r);
}

fn main() {
    let mut read = Reader::new(std::io::stdin().lock());
    let mut write = std::io::stdout().lock();
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
        // 7
        // 7 4 2 0 5 8 3
        // 4 0 0 1 3 1 3
        // "), "5");
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 4
        // 10 0 0 0
        // 2 0 0 0
        // "), "4");
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 4
        // 10 0 0 0
        // 3 0 0 0
        // "), "3");
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 4
        // 10 0 0 0
        // 100 0 0 0
        // "), "3");
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 4
        // 10 0 0 10
        // 100 0 0 100
        // "), "5");
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 6
        // 1 10 10 10 0 0
        // 0 3  2  1  0 0
        // "), "7");
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 6
        // 1 10 10 10 0 0
        // 0 1  2  3  0 0
        // "), "6");
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 5
        // 10 10 10 0 0
        // 1  2  3  0 0
        // "), "6");
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 3
        // 10 10 0
        // 1  2  0
        // "), "7");
    }
}
