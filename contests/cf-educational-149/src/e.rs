use std::collections::HashSet;

use contest_lib_rs::{io, emitln, mod_ring::ModNumber};

type ModNum = ModNumber<998_244_353>;

fn quick_pow(mut x: ModNum, mut n: u32) -> ModNum {
    let mut answer = ModNum::from(1);
    while n > 0 {
        if n & 1 == 1 {
            answer *= x;
        }
        x *= x;
        n >>= 1;
    }
    answer
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let k = read.u32();
    let two_k = 1usize << k;
    let a = read.vec_i32(two_k);
    let mut answer = ModNum::from(1);
    let mut factorials = vec![ModNum::from(1)];
    for i in 1..=two_k {
        factorials.push(factorials[i - 1] * ModNum::from(i as i32));
    }
    for m in 0..=k {
        let two_m = 1 << m;
        let mask = (two_m - 1) << (k - m);
        // let mut known_seeds: HashSet<_> = a.iter().take(two_m as usize).map(|x| x & (two_m - 1)).collect();
        let mut fixed_seeds = HashSet::new();
        let prev_end = if m == 0 { 0 } else { two_m / 2 };
        // let mut free_seeds = if m == 0 { 1 << k } else { 1 << (k - 1) };
        // let mut free_seeds = two_m;
        let mut free_seeds = 1 << m.saturating_sub(1);
        for &x in a[..(two_m as usize)].iter() {
            if x < 0 {
                continue;
            }
            if x > prev_end {
                // free_seeds -= 1 << (k - m);
                free_seeds -= 1;
            }
            let seed = (x - 1) & mask;
            if !fixed_seeds.insert(seed) {
                emitln!(write, 0);
                return;
            }
        }
        // TBD: same group bonus check

        // let free_seeds = (1 << m.saturating_sub(1)) - fixed_seeds.len() as u32;
        // answer *= factorials[(two_m / 2) as usize - fixed_seeds.len()];
        // answer *= factorials[free_seeds as usize];
        answer *= factorials[free_seeds as usize] * quick_pow(ModNum::from(1 << (k - m)), free_seeds as u32);
    }
    emitln!(write, answer);
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
        // assert_trimmed_eq!(&run_solver(solve, "2  1 2 3 4"), "0");
        // assert_trimmed_eq!(&run_solver(solve, "2  1 3 4 2"), "1");
        // assert_trimmed_eq!(&run_solver(solve, "1  -1 -1"), "2");
        // assert_trimmed_eq!(&run_solver(solve, "2  -1 -1 -1 -1"), "16");
        // assert_trimmed_eq!(&run_solver(solve, "3  -1 -1 -1 -1 2 -1 -1 -1"), "768");
        // assert_trimmed_eq!(&run_solver(solve, "0  1"), "1");

        // TBD:  -1 -1 3 4  test
    }
}
