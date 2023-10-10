use contest_lib_rs::relax::Relax;
use contest_lib_rs::{io, emitln};

const N: usize = 5;

fn ranom_value(a: &[u8]) -> i64 {
    const VALUES: &[i64] = &[1, 10, 100, 1000, 10000];
    let mut leftmost = [None; N];
    for i in (0..a.len()).rev() {
        let v = a[i] as usize;
        if leftmost[v] == None {
            leftmost[v] = Some(i);
        }
    }
    let mut sum = 0;
    for (i, v) in a.iter().enumerate() {
        let v = *v as usize;
        let mut sign = 1;
        for w in (v + 1)..N {
            if let Some(p) = leftmost[w] {
                if p > i {
                    sign = -1;
                    break;
                }
            }
        }
        sum += sign * VALUES[v];
    }
    sum
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let s = read.word();
    let mut a: Vec<_> = s.chars().map(|ch| ch as u8 - 'A' as u8).collect();
    let mut answer = ranom_value(&a);
    {
        let mut found = [false; N];
        for i in 0..a.len() {
            let v = a[i];
            if !found[v as usize] {
                found[v as usize] = true;
                for w in (v + 1)..(N as u8) {
                    a[i] = w;
                    answer.relax_max(ranom_value(&a));
                }
                a[i] = v;
            }
        }
    }
    {
        let mut found = [false; N];
        for i in (0..a.len()).rev() {
            let v = a[i];
            if !found[v as usize] {
                found[v as usize] = true;
                for w in 0..v {
                    a[i] = w;
                    answer.relax_max(ranom_value(&a));
                }
                a[i] = v;
            }
        }
    }
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
    use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        4
        DAAABDCA
        AB
        ABCDEEDCBA
        DDDDAAADDABECD
        "), "\
        11088
        10010
        31000
        15886");
    }
}
