// HACKED

use contest_lib_rs::io::prelude::*;

fn balance(v: &[u8]) -> i32 {
    let mut n0 = 0;
    let mut n1 = 0;
    let mut b = 0;
    for &x in v {
        match x {
            0 => {
                b += n1;
                n0 += 1;
            }
            1 => {
                b -= n0;
                n1 += 1;
            }
            _ => unreachable!()
        }
    }
    b
}

fn parse_str(s: &str) -> Vec<u8> {
    s.chars().map(|ch| if ch == '0' { 0 } else { 1 }).collect()
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let mut v = parse_str(&read.word());
    let n = v.len();
    let mut swaps = 0;
    let mut bal = balance(&v);
    while bal != 0 {
        swaps += 1;
        let p1;
        let p2;
        if bal > 0 {
            p1 = v.iter().position(|&x| x == 1).unwrap();
            p2 = v.iter().rposition(|&x| x == 0).unwrap();
        } else {
            p1 = v.iter().position(|&x| x == 0).unwrap();
            p2 = v.iter().rposition(|&x| x == 1).unwrap();
        }
        v.swap(p1, p2);
        let new_bal = balance(&v);
        if new_bal.signum() != bal.signum() {
            v.swap(p1, p2);
            let mut swap_found = false;
            for i in 0..n {
                for j in 0..i {
                    if v[i] != v[j] {
                        v.swap(i, j);
                        if balance(&v) == 0 {
                            swap_found = true;
                        }
                        v.swap(i, j);
                    }
                }
            }
            if !swap_found {
                swaps += 1;
            }
            break;
        }
        bal = new_bal;
    }
    emitln!(write, swaps);
}

fn main() {
    // println!("{}", balance(&parse_str("11001100")));
    // println!("{}", balance(&parse_str("01001101")));
    // return;

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
        assert_trimmed_eq!(&run_solver(solve, "101"), "0");
        assert_trimmed_eq!(&run_solver(solve, "1000110"), "0");
        assert_trimmed_eq!(&run_solver(solve, "11010"), "1");
        assert_trimmed_eq!(&run_solver(solve, "11001100"), "2");

        assert_trimmed_eq!(&run_solver(solve, "111"), "0");
        // assert_trimmed_eq!(&run_solver(solve, "11111111111111110000000000000000"), "");
    }
}
