use contest_lib_rs::io::prelude::*;

fn fill(b: &mut Vec<i32>, mut l: usize, mut r: usize, mut vl: i32, mut vr: i32) -> Result<(), ()> {
    if vl < 0 && vr < 0 {
        for i in l..r {
            b[i] = i as i32 % 2 + 1;
        }
        Ok(())
    } else if vl < 0 {
        for i in (l..r).rev() {
            if vr == 1 {
                vr *= 2;
            } else {
                vr /= 2;
            }
            b[i] = vr;
        }
        Ok(())
    } else if vr < 0 {
        for i in l..r {
            if vl == 1 {
                vl *= 2;
            } else {
                vl /= 2;
            }
            b[i] = vl;
        }
        Ok(())
    } else {
        while l < r {
            if vl > vr {
                vl /= 2;
                b[l] = vl;
                l += 1;
            } else if vr > vl {
                vr /= 2;
                r -= 1;
                b[r] = vr;
            } else {
                vl *= 2;
                b[l] = vl;
                l += 1;
            }
        }
        if vl / 2 == vr || vr / 2 == vl {
            Ok(())
        } else {
            Err(())
        }
    }
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_i32(n);
    let mut b = a.clone();

    let mut start = None;
    for (i, &x) in a.iter().enumerate() {
        if let Some((l, vl)) = start {
            if x > 0 {
                let r = i;
                let vr = a[i];
                if fill(&mut b, l, r, vl, vr).is_err() {
                    emitln!(write, -1);
                    return;
                }
                start = None;
            }
        } else {
            if x == -1 {
                if i > 0 {
                    start = Some((i, a[i - 1]));
                } else {
                    start = Some((0, -2));
                }
            }
        }
    }
    if let Some((l, vl)) = start {
        let r = n;
        let vr = -2;
        if fill(&mut b, l, r, vl, vr).is_err() {
            emitln!(write, -1);
            return;
        }
    }
    assert!(b.iter().all(|&x| x > 0));
    for i in 0..(n - 1) {
        if !(b[i] / 2 == b[i + 1] || b[i + 1] / 2 == b[i]) {
            emitln!(write, -1);
            return;
        }
    }
    emitln!(write, b);
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
        assert_trimmed_eq!(&run_solver(solve_case, "8  -1 -1 -1 2 -1 -1 1 -1"), "1 2 1 2 1 2 1 2");
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 9
        // 8
        // -1 -1 -1 2 -1 -1 1 -1
        // 4
        // -1 -1 -1 -1
        // 6
        // 3 -1 -1 -1 9 -1
        // 4
        // -1 5 -1 6
        // 4
        // 2 -1 -1 3
        // 4
        // 1 2 3 4
        // 2
        // 4 2
        // 5
        // -1 3 -1 3 6
        // 13
        // -1 -1 3 -1 -1 -1 -1 7 -1 -1 3 -1 -1
        // "), "\
        // 4 9 4 2 4 2 1 2
        // 7 3 6 13
        // 3 1 2 4 9 18
        // -1
        // -1
        // -1
        // 4 2
        // 6 3 1 3 6
        // 3 1 3 1 3 7 3 7 3 1 3 1 3");
    }
}
